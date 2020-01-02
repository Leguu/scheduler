# Scheduler Application

Welcome to the Rust Scheduler app. This will serve as a very quick guide on how to read this project best. This project is split into 3 main components; the `clock` module, the `gui` module, and the `application` module. The `clock` and `application` modules are the 'backend', so to speak, and contain most of the logic for this program. The `gui` module contains a lot of code written for `GTK`, which is a foreign library, and is not commented as there is a lot of unavoidable boilerplate code.

The contents of each module:

`clock` module:
- Date struct (for 2019-11-05)
- Day struct  (for Thursday)
- Time struct (for 09:30)

`application` module:
- Application struct (contains a list of courses and holidays)
- Course struct (contains Date and Task)
- Task struct (contains Date)

For more information on how these structures work together, see the comments in their respective files. The 'TESTS' sections in the files only contain tests and no logic. However, they may help you understand how a certain function is meant to work.

**IF YOU'RE GOING TO CHECK THE `gui` MODULE, THEN START AT `gui/mod.rs`. GOOD LUCK!**

Everything in the `target/` directory is generated by Cargo, and is NOT necessary for viewing. The `Cargo.toml` file contains a list of dependencies and some basic information about the project (name, description). The `Cargo.lock` file is a lock file, as the name suggests. It's generated by Cargo and is not necessary for viewing. `.gitignore` is a file for the git versioning system. The `.git/` directory is generated by git, and is not necessary for viewing. I have included a example `scheduler.bin` file, which is the data file for the application. Feel free to remove it and test the application from scratch!

Hint: run `find . -name '*.rs' | xargs wc -l` if you want the line-count of this project.
