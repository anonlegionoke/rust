/*
 * There are differnt types of iterators and different ways to use a single type of iterators.
 *
 * Mainly, iter(), iter_mut(), into_iter()
 *
 * Among this, into_iter() is what happening by default when you do:
 *  
 * /// 
 *  let nums = vec![1, 2, 3, 4]
 *
 *  for val in nums {
 *      println!("Value {}", val)
 *  }
 *
 *  You can't print nums any more here, because the ownership if nums has moved to into_iter().
 *  ///
 *
 *  ---------------------------------------------------------------------------------------------------
 *
 *
 *  To make nums printable even after iteration do below
 *  
 *  ///
 *      let nums = vec![1, 2, 3, 4];
 *
 *      for val in nums.iter() {
 *          println!("val is {}", val)
 *      }
 *      println!("nums is {:?}", nums);
 *  ///
 *
 *  ----------------------------------------------------------------------------------------------------
 *
 *
 *  To make it mutable inside iteration,
 *
 *  ///
 *      let mut nums = vec![1, 2, 3, 4];
 *      for val in nums.iter_mut() {
 *          *val = *val + 1;
 *          println!("val is {}", val)
 *      }
 * */



---------------------------------------------------------------------------------------------------------------------------------

/* Consuming Adaptors */

// Adapators are methods we can call on iterators [eg: v1.iter().sum()], but the consuming types do take up the
// ownership of the v1 used.

fn main() {
    let v1 = vec![1, 2, 3]

        let v1_iter = v1.iter();

        let total = vi_iter.sum(); // 6
                                   
        // Now, here you can print v1 or total but not v1_iter because the sum() method took
        // ownership of it when consumed to do the sum calculation.

}


/* Iterator Adaptors */

// This type of methods do not consume the iterator, there for no ownership of the iterator, but
// they do change some aspects of the OG iterator to produce different iterators from it.

// 1.
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter1 = v1.iter();

    let v1_iter2 = v1_iter1.map(|x| x + 1);

    for x in v1_iter2 {
        println!("{}", x);
    }
}

// 2.
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter1 = v1.iter();

    let v1_iter2 = v1_iter1.filter(|x| *x % 2 != 0);

    for x in v1_iter2 {
        println!("{}", x);
    }
}

/* Q. Write the logic to first filter all odd values then double each value and create new vector */


