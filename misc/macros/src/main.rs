use std::collections::HashMap;

fn main() {

    // if_any!(1==0 1==1; {println!("some")});
    // let a = expand1!(row <usize> in 1..3, col <usize> in 2..4, {println!("{}{}", row, col)});
    // println!("{}", maths!(5, plus, (3 + 6)));
    // let a = vec1![1, 2, 3, 4,];
    // let a = hashmap1!(5 => 3; 6=>2;);
    let a = nested1!(5 -> (1,2,3,); 3 -> (4,5,););


    println!("{:?}", a)


}

// #[macro_export]
// macro_rules! test1 {
//     ($a:expr) => {$a};
// }

#[macro_export]
macro_rules! nested1 {
    ($($parent:literal -> ($($val:literal, )*); )+) => {
        {
            let mut pars: Vec<usize> = Vec::new();
            $(pars.push($parent));+;
            let mut graph: Vec<Vec<usize>> = vec![Vec::new(); pars.iter().max().unwrap().to_owned() + 1];


            $( $( {graph[$parent].push($val) } )*; )*;

            // $($(graph[$parent].push($val))*)*

            graph
        }
    };
}

#[macro_export]
macro_rules! hashmap1 {
    ($($key:literal => $value:literal; )+) => {
        {
            let mut table = HashMap::new();
            $(table.insert($key, $value));+;
            table
        }
    };

}

#[macro_export]
macro_rules! vec1 {
    ($($my_literal:literal,)*) => {
        {
            let mut my_vec = Vec::new();
            $(my_vec.push($my_literal));*;
            my_vec
        }
    }
}

#[macro_export]
macro_rules! add {
    ($a:literal plus $b:literal) => {
        $a+$b
    };
}

#[macro_export]
macro_rules! maths {
    ($a:expr, plus, $b:expr) => {
        $a + $b

    };
}

#[macro_export]
macro_rules! expand1 {
    ($row:ident <$a:ty> in $a1:literal..$a2:literal, $col:ident <$b:ty> in $b1:literal..$b2:literal, $c:block) => {
        for row in $a1..$a2 {
            let $row: $a = row;
            for col in $b1..$b2 {
                let $col: $b = col;
                $c
            }
        }
    };
}

#[macro_export]
macro_rules! if_any {
    ($($condition:expr )*; $code:block) => {
        if $($condition ||)* false {
            $code

        }
};
}