# PetBox: Rust Tamagotchi Virtual Pet Simulator 🐾

This is a virtual pet simulator based on the classic Tamagotchi game. The application allows users to care for a virtual pet that has states of hunger and happiness that change over time.

## Description

This project is an implementation of the famous Tamagotchi game where you need to take care of your virtual pet. The pet has hunger and happiness states that change over time, and you must interact with it to keep it happy and well-fed.

## Features

- **Interactive virtual pet**: Your Tamagotchi has a name, happiness, and hunger levels.
- **Real-time state system**: States change automatically every second.
- **Available actions**:
  - 🎮 Play with your Tamagotchi (increases happiness)
  - 🍎 Feed your Tamagotchi (reduces hunger)
  - 👋 Exit the game
- **Console interface**: Interactive menu and state updated in real-time.
- **Multithreading system**: Automatic state update without blocking user input.

## Project Structure
```
src/
├── main.rs # Entry point and main logic
├── constants.rs # Game constants (initial values, increments, etc.)
├── utils.rs # Utilities (user input, screen clearing)
└── models/
├── mod.rs # Models module
└── tamagotchi.rs # Tamagotchi structure and logic
```

## Installation and Execution

### Requirements
- Rust (2021 edition or higher)
- Cargo

### Running the project
```bash
cargo run
```
### How to Play
- Start: Enter the name of your Tamagotchi.

- Main Menu: You will see three options:

  - 1 - Play with your Tamagotchi

  - 2 - Feed your Tamagotchi

  - 3 - Exit the game

- State: Your pet's state updates automatically every second.

- Care: Keep your Tamagotchi happy by playing with it and feed it when it's hungry.

## Game Mechanics
### States
- Happiness: Starts at 100, decreases when the Tamagotchi gets very hungry.

- Hunger: Starts at 0, gradually increases over time.

### Actions
- Play: Increases happiness by 10 points and may slightly increase hunger.

- Feed: Reduces hunger by 10 points.

- Time: Every second, hunger increases by 1 point.

### Alerts
- When hunger exceeds 80 points, your Tamagotchi will be hungry, and its happiness will begin to decrease.

### Configuration
You can modify the game's values by editing the constants in src/constants.rs:
```
pub const INITIAL_HUNGER: u32 = 0;           // Initial hunger
pub const INITIAL_HAPPINESS: u32 = 100;     // Initial happiness
pub const HUNGER_WARNING: u32 = 80;         // Critical hunger threshold
pub const PLAY_HAPPINESS_INCREASE: u32 = 10; // Happiness increase from playing
// ... more constants
```

## Technical Architecture
### Concurrency
The project uses Arc<Mutex<T>> to safely share the Tamagotchi state between threads:

Main thread: Handles user input.

Secondary thread: Updates the state every second and displays the interface.

### Modules
- models::tamagotchi: Contains the Tamagotchi structure and its methods.
- utils: Utility functions for input/output.
- constants: Game constant values.

### Contributing
If you want to contribute to the project:

- Fork the repository

- Create a branch for your feature (git checkout -b new-feature)

- Commit your changes (git commit -am 'Add new feature')

- Push to the branch (git push origin new-feature)

- Create a Pull Request
### Future Improvements
- Save/load system

- Multiple food types

- Different play activities

- Aging system

- Graphical interface

- Sounds and effects

### License
This project is licensed under the MIT License. See the LICENSE file for more details.

Enjoy taking care of your virtual Tamagotchi! 🎮✨
