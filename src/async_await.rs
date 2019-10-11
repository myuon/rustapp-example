use futures::{Future as Future3, FutureExt, TryFutureExt};

pub fn wrap<F, U, T, Ok, Error>(
  f: F,
) -> impl Fn(U) -> Box<dyn futures01::Future<Item = Ok, Error = Error>> + Clone + 'static
where
  Ok: 'static,
  Error: 'static,
  F: Fn(U) -> T + Clone + 'static,
  T: Future3<Output = Result<Ok, Error>> + 'static,
{
  move |u| {
    // Turn a future3 Future into futures1 Future
    let fut1 = f(u).boxed_local().compat();
    Box::new(fut1)
  }
}
