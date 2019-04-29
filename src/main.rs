mod loan_revolver;

use std::io::Write;
use std::str::{FromStr, ParseBoolError};
use std::slice::Iter;
use crate::loan_revolver::{LoanRevolver, LoanRevolverError};

// loan_revolver a 1000000 45000
// loan_revolver c 1000000 60
fn main() {
    match work(std::env::args().skip(1).collect()) {
        Ok(r) => {
            writeln!(std::io::stdout(), "{}", r).unwrap();
            std::process::exit(0);
        },
        Err(e) => {
            writeln!(std::io::stderr(), "{:?}", e).unwrap();
            std::process::exit(1);
        },
    };
}

fn work(args: Vec<String>) -> Result<String, String> {
    if args.len() != 4 {
        return Err(r"
            Usage: loan_revolver a TOTAL_DEBT_AMOUNT ANNUAL_INTEREST_RATE REPAYMENT_AMOUNT
                   loan_revolver t TOTAL_DEBT_AMOUNT ANNUAL_INTEREST_RATE REPAYMENT_TIMES
        ".to_string());
    }

    let mut args = args.iter();
    let mode = args.next().unwrap().chars().next().unwrap();
    let debt = match usize::from_str(args.next().unwrap()) {
        Ok(n) => n,
        Err(e) => return Err(format!("invalid debt {}", e))
    };
    let interest = match f64::from_str(args.next().unwrap()) {
        Ok(n) => n,
        Err(e) => return Err(format!("invalid interest {}", e))
    };
    let repayment = match usize::from_str(args.next().unwrap()) {
        Ok(n) => n,
        Err(e) => return Err(format!("invalid repayment {}", e))
    };
    match match mode {
        'a' => LoanRevolver::by_per_month(debt, interest, repayment),
        't' => LoanRevolver::by_times(debt, interest, repayment),
        _ => Err(LoanRevolverError::Other("invalid Mode".to_string())),
    } {
        Ok(r) => Ok(r.to_json()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
