# auto-connect-rs
E6 Connect Bot

Currently only supports 1920 x 1080 resolution, but this will be fixed in one of the next updates.

This is a bot that can play rounds of Stroke Play in E6 Connect on a loop. 
I recommend that you choose courses with fewer water hazards, as this is one of the major current limitations.

Usage:

If you have the Rust toolset installed already, you can clone the repo and run it with a round of stroke play loaded. If the Rust toolset is not installed, 
put the auto-connect-rs.exe in a folder with all of the images, and run it via the command line. Ctrl + C to kill the bot.

Limitations:

* Handling some water hazard situations.Since this is an external tool that uses image recognition to determine game state,
this is probably never going to be perfect given that the water is never exactly identical to a template image. That being said, I do have an idea to improve this in the next update.

* As stated above, the only resolution currently supported is 1080p. I have a solution implemented to normalize pixel coordinates on screen, and am testing it now.

* Other game modes that have unqiue menus, such as Scramble, and many of the games, have 95 % of the work done to eventually support playing every mode on a loop, 
but as of now you can only do stroke play, and must start at the tee of an already loaded round, rather than in the main menu.

* The final limitation is my experience with programming in Rust. I had a lot of fun writing this, but I am certainly not an expert at Rust programming.
I could have used Traits to avoid some serious duplication issues with the code here, but I wanted to battle through a rough draft to get the basics of Rust down, before improving.

Feature Roadmap:
================

* Add logging both in the form of log files, and saving screenshots if something unusual is detected on screen.
* Improve image recognition to the point that serious anomoly detection can be done, with photos of the unusual occurance taking play on screen saved. This will be combined with,
log output to attempt to automate the process of finding bugs, and saving the repro steps that got us there.

