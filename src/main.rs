use async_recursion::async_recursion;
use futures::{future::join_all};
use rand::Rng;
use tokio::time::{sleep, timeout, Duration, Sleep};

#[tokio::main]
async fn main() {
    let name = "Sailor";
    let english = sleep_print("Hello", name);
    let french = sleep_print("Bonjour", name);
    let spanish = sleep_print("Hola", name);

    let combined = join_all(vec![english, french, spanish]);

    // And now, this `await` will start the effect:
    let timeout_result = timeout(Duration::from_secs(5), combined).await;
    assert_eq!(timeout_result.is_err(), true); // check the operation did indeed timeout
}

#[async_recursion]
async fn sleep_print(word: &str, name: &str) -> () {
    random_sleep().await;
    println!("{word}, {name}");
    sleep_print(word, name).await;
}

fn random_sleep() -> Sleep {
    let ms = rand::thread_rng().gen_range(200..700);
    sleep(Duration::from_millis(ms))
}

/*
import cats.effect.{IO, IOApp}
import cats.effect.std.Random

import scala.concurrent.duration._

object Hello extends IOApp.Simple {

  def sleepPrint(word: String, name: String, rand: Random[IO]) =
    for {
      delay <- rand.betweenInt(200, 700)
      _     <- IO.sleep(delay.millis)
      _     <- IO.println(s"$word, $name")
    } yield ()

  val run =
    for {
      rand <- Random.scalaUtilRandom[IO]

      // try uncommenting first one locally! Scastie doesn't like System.in
      // name <- IO.print("Enter your name: ") >> IO.readLine
      name <- IO.pure("Daniel")

      english <- sleepPrint("Hello", name, rand).foreverM.start
      french  <- sleepPrint("Bonjour", name, rand).foreverM.start
      spanish <- sleepPrint("Hola", name, rand).foreverM.start

      _ <- IO.sleep(5.seconds)
      _ <- english.cancel >> french.cancel >> spanish.cancel
    } yield ()
}
*/
