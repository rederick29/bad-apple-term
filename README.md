# bad-apple-term

My attempt at making an app that plays the bad apple video in the terminal in about 100 lines of rust. 'q' to quit while it is running.

The app gets the video frames by looking for bmp images in a folder named "images" in the repo root ("./images/"). The images should be named 1.bmp, 2.bmp ... n.bmp where n is the frame number. Bmp is the assumed as the default file extension but you may provide another as the first argument to the program. It can play any video (sequence of frames). Aspect ratio of the frames is not kept and is replaced by the ratio of your terminal. Frame rate can be controlled using a constant variable in main.rs.
