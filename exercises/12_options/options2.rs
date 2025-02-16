fn main() {
    // You can optionally experiment here.
    simple_option();
    layered_option();
}

fn simple_option() {
    let target = "rustlings";
    let optional_target = Some(target);

    // TODO: Make this an if-let statement whose value is `Some`.
    if let Some(_) = optional_target {
        println!("It's the same as optional_target!");
    } else {
        println!("It's not the same as optional_target :(");
    }
}

fn layered_option() {
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }

    let mut cursor = range;

    // TODO: Make this a while-let statement. Remember that `Vec::pop()`
    // adds another layer of `Option`. You can do nested pattern matching
    // in if-let and while-let statements.
    while let Some(Some(i)) = optional_integers.pop() {
        if i == 0 {
            println!("Reached 0! Loop finished");
            break;
        } else {
            println!("Cursor = {}", cursor);
            cursor -= 1;
        }
    }

    // Handle case where we might encounter None at the end
    if let Some(None) = optional_integers.pop() {
        println!("None from integers");
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn simple_option() {
//         let target = "rustlings";
//         let optional_target = Some(target);

//         // TODO: Make this an if-let statement whose value is `Some`.
//         if let Some(i) = optional_target {
//             println!("It's the same as optional_target!");
//         } else {
//             println!("It's not the same as optional_target :(");
//         }
//         word = optional_target {
//             assert_eq!(word, target);
//         }
//     }

//     #[test]
//     fn layered_option() {
//         let range = 10;
//         let mut optional_integers: Vec<Option<i8>> = vec![None];

//         for i in 1..=range {
//             optional_integers.push(Some(i));
//         }

//         let mut cursor = range;

//         // TODO: Make this a while-let statement. Remember that `Vec::pop()`
//         // adds another layer of `Option`. You can do nested pattern matching
//         // in if-let and while-let statements.
//         integer = optional_integers.pop() {
//             assert_eq!(integer, cursor);
//             cursor -= 1;
//         }

//         assert_eq!(cursor, 0);
//     }
// }
