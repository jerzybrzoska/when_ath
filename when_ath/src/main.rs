fn main() {

//let years_alltogether = DAYS_ALLTOGETHER / DAYS_IN_Y ;

//const YEARS_FLOOR: f32 = YEARS_ALLTOGETHER.floor() ; //calls in constants are limited to constant functions, tuple structs and tuple variants
//let years_floor: f32 = YEARS_ALLTOGETHER.floor() ; //calls in constants are limited to constant functions, tuple structs and tuple variants
//let years_floor: f32 = years_alltogether.floor() ; //calls in constants are limited to constant functions, tuple structs and tuple variants

//let y = YEARS_ALLTOGETHER.floor();
/*
const HIGH2021: When = When {
//	year: HALVING2020.year + years_floor, // error[E0435]: attempt to use a non-constant value in a constant
	year: HALVING2020.year + YEARS_ALLTOGETHER, // 
//	year: HALVING2020.year + &years_floor, // error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
    month: 12.0,
    day: 20.0,
	}; 
	*/
	
let high2021: When = When {
	year: HALVING2020.year + (MONTHS_ALLTOGETHER) / 12.0,
	month: DAYS_FROM_DM / DAYS_IN_M + 5.0 ,
    day: 20.0 + NO_OF_DAYS_FROM_DAYS ,
	
	};	

let low2015: Low = Low {
	high: 1128.4, 
	low: 197.84 ,
	percentage: (1128.4 - 197.84 ) / 1128.4,
};	
	
println!("The all-time high for Bitcoin will happen on {:.0}/{:.0}/{:.0} i.e. {:.1} months from its halving in May of 2020.", high2021.day, high2021.month, high2021.year, MONTHS_ALLTOGETHER );

let low2018: Low = Low {
	high: 19798.6, 
	low: 3156.2,
	percentage: (19798.6 - 3156.2 ) / 19798.6,
};

let low2022: Low = Low {
	high: ( 19798.6 / 1128.4 ) * 19798.6 , 
	low: (1.0 - 0.8326284) * ( 19798.6 / 1128.4 ) * 19798.6,
	percentage: (0.82467216 + 0.8405847 ) / 2.0,
};

println!("The all-time high of ${:.0} for Bitcoin was followed by the drop of {:.1}%.", low2015.high, low2015.percentage * 100.0 );


println!("The last all-time high of {:.0} for Bitcoin was followed by the drop of {:.1}%.", low2018.high, low2018.percentage * 100.0 );

println!("The high of 2017 was {:.2} times higher than the high of 2014. The average of the last two corrections is {:.1}%.", low2022.high / low2018.high, low2022.percentage * 100.0 );

println!("Should the high of 2021 be {:.2} times higher than the high of 2017 it would be at the price of ${:.0}.", low2022.high / low2018.high, low2022.high );

println!("Should the correction in the 2022-2023 be that of {:.2}% from the top of {:.0}, the bottom would be at ${:.0}.", low2022.percentage * 100.0, low2022.high, low2022.low );

}

const DAYS_IN_Y: f32 = 365.25;

const DAYS_IN_M: f32 = 30.43757;


struct When {
    year: f32,
    month: f32,
    day: f32,
}

struct Low {
	high: f32,
	low: f32,
	percentage: f32  
}

const HALVING2016: When = When {
		year: 2016.0,
    month: 7.0,
    day: 9.0,
	};
	
const HIGH2017: When = When {
		year: 2017.0,
    month: 12.0,
    day: 20.0,
	};

const HALVING2020: When = When {
		year: 2020.0,
    month: 5.0,
    day: 18.0,
	};
	

	
const NO_OF_DAYS_FROM_YEARS: f32 =   (HIGH2017.year - HALVING2016.year) * DAYS_IN_Y ; 

const NO_OF_DAYS_FROM_MONTHS: f32 =   (HIGH2017.month - HALVING2016.month) * DAYS_IN_M ; //This number can be negative!

const NO_OF_DAYS_FROM_DAYS: f32 = HIGH2017.day - HALVING2016.day ; //This number can be negative!

const DAYS_FROM_DM: f32 = DAYS_ALLTOGETHER - DAYS_IN_Y ;


const DAYS_ALLTOGETHER: f32 = NO_OF_DAYS_FROM_YEARS + NO_OF_DAYS_FROM_MONTHS + NO_OF_DAYS_FROM_DAYS;

const MONTHS_ALLTOGETHER: f32 = DAYS_ALLTOGETHER / DAYS_IN_M ;

//const YEARS_ALLTOGETHER: f32 = DAYS_ALLTOGETHER / DAYS_IN_Y ;


//https://www.cmcmarkets.com/en/learn-cryptocurrencies/bitcoin-When
//https://www.ig.com/en/bitcoin-btc/bitcoin-When#:~:text=The%20next%20bitcoin%20When%20is,from%2012.5%20to%206.25%20bitcoins.
