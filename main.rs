use std::f64::consts::PI;

fn main() {
	const G: f64 = 6.67430e-11; // м³ / (кг·с²)
	// Земля
	const EARTH_MASS: f64 = 5.972e24;      // кг
	const EARTH_RADIUS: f64 = 6_371_000.0; // м
	// Луна
	const MOON_MASS: f64 = 7.342e22;
	const MOON_RADIUS: f64 = 1_737_400.0;
	// Марс
	const MARS_MASS: f64 = 6.4171e23;
	const MARS_RADIUS: f64 = 3_389_500.0;
	// Юпитер
	const JUPITER_MASS: f64 = 1.8982e27;
	const JUPITER_RADIUS: f64 = 69_911_000.0;
	// Сатурн
	const SATURN_MASS: f64 = 5.6834e26;
	const SATURN_RADIUS: f64 = 58_232_000.0;
    
	println!(r"                                         _.oo.
                 _.u[[/;:,.         .odMMMMMM'
              .o888UU[[[/;:-.  .o@P^    MMM^
             oN88888UU[[[/;::-.        dP^
            dNMMNN888UU[[[/;:--.   .o@P^
           ,MMMMMMN888UU[[/;::-. o@^          
           NNMMMNN888UU[[[/~.o@P^          
   ____    ____    ____    ___   _____    ____      _      _        ____
  / __ \  |  _ \  | __ )  |_ _| |_   _|  / ___|    / \    | |      / ___|
 | |  | | | |_) | |  _ \   | |    | |   | |       / _ \   | |     | |
 | |  | | |  _ <  | |_) |  | |    | |   | |___   / ___ \  | |___  | |___
  \____/  |_| \_\ |____/  |___|   |_|    \____| /_/   \_\ |_____|  \____|
           888888888UU[[[/o@^-..
          oI8888UU[[[/o@P^:--..
       .@^  YUU[[[/o@^;::---..
     oMP     ^/o@P^;:::---..
  .dMMM    .o@^ ^;::---...
 dMMMMMMM@^`       `^^^^
YMMMUP^
 ^^");
	println!("-------------------------------------------------------------------------");
	loop { 
 		println!("Выберите планету: \n1. Земля
2. Луна
3. Марс
4. Юпитер
5. Сатурн");
		let mut planet_st = String::new();
		std::io::stdin().read_line(&mut planet_st).unwrap();
		let planet: i32 = planet_st.trim().parse().unwrap();
		let (name, mass, radius) = match planet {
			1 => ("Земля", EARTH_MASS, EARTH_RADIUS),
			2 => ("Луна", MOON_MASS, MOON_RADIUS),
			3 => ("Марс", MARS_MASS, MARS_RADIUS),
			4 => ("Юпитер", JUPITER_MASS, JUPITER_RADIUS),
			5 => ("Сатурн", SATURN_MASS, SATURN_RADIUS),
    		_ => {
        		println!("Такой планеты нет");
				return;
    		}
		};
		println!("");
		println!("Введите высоту орбиты(км): ");
		let mut height_st = String::new();
		std::io::stdin().read_line(&mut height_st).unwrap();
		let height: f64 = height_st.trim().parse().unwrap();
		let height_m = height * 1000.0;
		let orbit_radius = radius + height_m;
		let velocity = (G * mass / orbit_radius).sqrt();
		let mut period = 2.0 * PI * orbit_radius / velocity;
		let period = period / 60.0;
		let period_day = 1440.0 / period;
		println!("\nПланета: {}", name);
		println!("Орбитальная скорость: {:.2} м/с", velocity);
		println!("Период обращения: {:.2} мин", period);
		println!("Кол-во оборотов за сутки: {:.2}\n", period_day);
	}
}	






