fn memo_fibo (memo: &mut Vec<i32>, n: usize) -> i32 {
  let mut indx:usize = memo.len();

  if indx < 1 {
    *memo = vec![0,1];
    indx = 2;
  }

  if indx <= n {
    for i in indx..n+1 {
      let a:i32 = memo[i-1];
      let b:i32 = memo[i-2];
      memo.insert(i, a + b);
    }
  }

  memo[n]
}

fn main () {
  let mut memo:Vec<i32> = vec![];
  let fib0:i32 = memo_fibo(&mut memo, 0);
  println!("{:?}", fib0);
  let fib1:i32 = memo_fibo(&mut memo, 5);
  println!("{:?}", fib1);
  let fib2:i32 = memo_fibo(&mut memo, 10);
  println!("{:?}", fib2);
}