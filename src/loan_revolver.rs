use crate::loan_revolver::LoanRevolverError::NotEnough;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub turn: usize,
    pub rest: f64,
    pub next_rest: f64,
    pub amount: f64,
    pub principal: f64,
    pub interest_amount: f64,
    pub next_interest_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanRevolver {
    pub total_amount: usize,
    pub total_interest_amount: usize,
    pub total_months: usize,
    pub annual_interest: f64,
    pub monthly_interest: f64,
    pub monthly_compound_interest: f64,
    pub rows: Vec<Row>,
}

#[derive(Debug)]
pub enum LoanRevolverError {
    NotEnough,
    Other(String),
}
 
impl LoanRevolver {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn by_per_month(total_raw: usize, annual_interest: f64, amount_raw: usize) -> Result<LoanRevolver, LoanRevolverError> {
        let mut amount = amount_raw as f64;
        let mut rest = total_raw as f64;
        let monthly_interest = Self::annual_to_monthly_simple_interest(annual_interest);
        let mut plan = vec![];
        let mut turn = 0;
        let next_interest_amount = compute_interest(rest, monthly_interest);

        if next_interest_amount >= amount {
            return Err(NotEnough);
        }

        plan.push(Row {
            turn,
            rest,
            amount: 0.0,
            next_rest: rest,
            principal: 0.0,
            interest_amount: 0.0,
            next_interest_amount,
        });

        while rest > 0.0 {
            turn += 1;
            let interest = plan.last().unwrap().next_interest_amount;
            let now_rest = rest + interest;
            let next_rest = match now_rest - amount {
                n if n > 0.0 => n,
                n => {
                    amount += n;
                    0.0
                },
            };

            plan.push(Row {
                turn,
                rest,
                amount,
                principal: amount - interest,
                next_rest,
                interest_amount: interest,
                next_interest_amount: compute_interest(next_rest, monthly_interest),
            });

            rest = next_rest;
        }

        Ok(
            LoanRevolver {
                total_amount: plan.iter().fold(0.0, |a, row| a + row.amount) as usize,
                total_interest_amount: plan.iter().fold(0.0, |a, row| a + row.interest_amount) as usize,
                total_months: plan.len() - 1,
                annual_interest: annual_interest as f64,
                monthly_compound_interest: Self::annual_to_monthly_compound_interest(annual_interest),
                monthly_interest: (monthly_interest * 100.0),
                rows: plan,
            }
        )
    }

    pub fn by_times(total_raw: usize, year_rate: f64, count: usize) -> Result<LoanRevolver, LoanRevolverError> {
        let amount = LoanRevolver::compute_per_month(total_raw, year_rate, count);
        Self::by_per_month(total_raw, year_rate, amount)
    }

    fn compute_per_month(total_raw: usize, year_rate: f64, raw_count: usize) -> usize {
        let total = total_raw as f64;
        let count = raw_count as i32;
        let rate = Self::annual_to_monthly_simple_interest(year_rate);
        let a = total * rate * (1.0 + rate).powi(count);
        let b = (1.0 + rate).powi(count) - 1.0;

        (a / b).round() as usize
    }

    fn annual_to_monthly_simple_interest(rate: f64) -> f64 {
        rate / 12.0 / 100.0
    }

    fn annual_to_monthly_compound_interest(rate: f64) -> f64 {
        let raw = ((rate / 100.0) + 1.0).powf(1.0 / 12.0) - 1.0;
        (raw * 10000.0).floor() / 100.0
    }
}

fn compute_interest(amount: f64, rate: f64) -> f64 {
    (amount * rate).floor()
}

mod test {
    use crate::loan_revolver::LoanRevolver;

    #[test]
    fn test_year_to_month_rate_2() {
        let c = LoanRevolver::annual_to_monthly_compound_interest(24.0);
        assert_eq!(c, 1.8);
    }

    #[test]
    fn test_year_to_month_rate() {
        assert_eq!(LoanRevolver::annual_to_monthly_simple_interest(18.0), 0.015);
    }

    #[test]
    fn test_revolver_count() {
        assert_eq!(LoanRevolver::compute_per_month(1000000, 18.0, 60), 25393);
    }

    #[test]
    fn test_by_count() {
        let result = LoanRevolver::by_times(1000000, 18.0, 11).unwrap();
        assert_eq!(result.total_months, 11);
    }

    #[test]
    fn test_revolver_amount() {
        let expected_amounts = [
            1000000.0,
            915000.0,
            828725.0,
            741155.0,
            652272.0,
            562056.0,
            470486.0,
            377543.0,
            283206.0,
            187454.0,
            90265.0,
            0.0,
        ];
        let result = LoanRevolver::by_per_month(1000000, 18.0, 100000).unwrap();
        let actual_amounts: Vec<f64> = result.rows.iter().map(|r| r.next_rest).collect();
        assert_eq!(result.total_amount, 1091618);
        assert_eq!(result.total_interest_amount, 91618);
        assert_eq!(result.total_months, 11);
        assert_eq!(expected_amounts[..], actual_amounts[..]);

        let expected_amounts = [
            1000000.0,
            965000.0,
            929475.0,
            893417.0,
            856818.0,
            819670.0,
            781965.0,
            743694.0,
            704849.0,
            665421.0,
            625402.0,
            584783.0,
            543554.0,
            501707.0,
            459232.0,
            416120.0,
            372361.0,
            327946.0,
            282865.0,
            237107.0,
            190663.0,
            143522.0,
            95674.0,
            47109.0,
            0.0,
        ];
        let result = LoanRevolver::by_per_month(1000000, 18.0, 50000).unwrap();
        let actual_amounts: Vec<f64> = result.rows.iter().map(|r| r.next_rest).collect();
        assert_eq!(result.total_amount, 1197815);
        assert_eq!(result.total_interest_amount, 197815);
        assert_eq!(result.total_months, 24);
        assert_eq!(expected_amounts[..], actual_amounts[..]);
    }
}
