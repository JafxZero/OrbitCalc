# 🪐 OrbitCalc

> Simple console orbital calculator written in Rust.

**OrbitCalc** is a small Rust program that calculates basic orbital parameters for different celestial bodies.

## 🚀 Features

- 🌍 Earth
- 🌙 Moon
- 🔴 Mars
- 🟠 Jupiter
- 🪐 Saturn

For a selected celestial body and orbital altitude, OrbitCalc calculates:

- Orbital velocity
- Orbital period
- Number of orbits per day

## 🧮 How it works

Orbital velocity is calculated using:

```text
v = √(GM / r)
```

where:

- `G` — gravitational constant
- `M` — mass of the celestial body
- `r` — distance from the center of the celestial body

Orbital period:

```text
T = 2πr / v
```

## 📦 Installation

Make sure you have Rust installed:

```bash
rustc --version
```

Clone the repository:

```bash
git clone https://github.com/JafxZero/OrbitCalc.git
cd OrbitCalc
```

Run the project:

```bash
cargo run
```

## 💻 Example

```text
Выберите планету:
1. Земля
2. Луна
3. Марс
4. Юпитер
5. Сатурн

Введите высоту орбиты(км):
400

Планета: Земля
Орбитальная скорость: 7672.59 м/с
Период обращения: 92.41 мин
Кол-во оборотов за сутки: 15.58
```

## 🛠 Built with

- Rust
- Standard Library

## 📌 Project status

OrbitCalc is currently a small educational project and may receive new celestial bodies, calculations and input validation in future versions.

## 📄 License

This project is open source and available under the MIT License.
