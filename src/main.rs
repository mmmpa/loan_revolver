fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct RevolverRow {
    turn: usize,
    total: f64,
    amount: f64,
    rest: f64,
    addition: f64,
}

#[derive(Debug)]
struct Revolver {
    total: f64,
    count: usize,
    rows: Vec<RevolverRow>,
}

fn revolver_count(total_raw: usize, year_rate: usize, count: i32) -> usize {
    let total = total_raw as f64;
    let rate = year_to_month_rate(year_rate);
    let a = total * rate * (1.0 + rate).powi(count);
    let b = (1.0 + rate).powi(count) - 1.0;

    (a / b).round() as usize
}

fn revolver_amount(total_raw: usize, year_rate: usize, mut amount: f64) -> Revolver {
    let mut total = total_raw as f64;
    let rate = year_to_month_rate(year_rate);
    let mut plan = vec![];
    let mut turn = 0;

    plan.push(RevolverRow {
        turn,
        total,
        amount: 0.0,
        rest: total,
        addition: (total * rate).round()
    });

    while total > 0.0 {
        turn += 1;

        total += (total * rate).floor();
        let rest = match total - amount {
            n if n > 0.0 => n,
            n => {
                amount += n;
                0.0
            },
        };

        plan.push(RevolverRow {
            turn,
            total,
            amount,
            rest,
            addition: (rest * rate).round()
        });

        total = rest;
    }

    Revolver {
        total: 0.0,
        count: plan.len(),
        rows: plan,
    }
}

fn year_to_month_rate(rate: usize) -> f64 {
    (rate as f64) / 12.0 / 100.0
}

#[test]
fn test_year_to_month_rate() {
    assert_eq!(year_to_month_rate(18), 0.015);
}

#[test]
fn test_revolver_count() {
    assert_eq!(revolver_count(1000000, 18, 60), 25393);
}

#[test]
fn test_revolver_amount() {
    let expected = [
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
    let actual: Vec<f64> = revolver_amount(1000000, 18, 100000.0).rows.iter().map(|r| r.rest).collect();
    assert_eq!(expected[..], actual[..]);

    let expected = [
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
    let actual: Vec<f64> = revolver_amount(1000000, 18, 50000.0).rows.iter().map(|r| r.rest).collect();
    assert_eq!(expected[..], actual[..]);
}
