use rand::Rng;

pub fn create_bot_name() -> String {
  let names = [
    "Wade",
    "Dave",
    "Seth",
    "Ivan",
    "Riley",
    "Gilbert",
    "Jorge",
    "Dan",
    "Brian",
    "Roberto",
    "Ramon",
    "Miles",
    "Liam",
    "Nathaniel",
    "Ethan",
    "Lewis",
    "Milton",
    "Claude",
    "Joshua",
    "Glen",
  ];
  let surnames = [
    "Williams", "Harris", "Thomas", "Robinson", "Walker", "Scott", "Nelson", "Mitchell", "Morgan",
    "Cooper", "Howard", "Davis", "Miller", "Martin", "Smith", "Anderson", "White", "Perry",
    "Clark", "Richards",
  ];
  let mut rng = rand::thread_rng();
  format!(
    "{} {}",
    names[rng.gen_range(0..names.len())],
    surnames[rng.gen_range(0..surnames.len())]
  )
}
