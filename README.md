# bad-apple-term

My attempt at making an app that plays the bad apple video in the terminal in about 100 lines of rust. 'q' to quit while it is running.

The app gets the video frames by looking for bmp images in a folder named "images" in the repo root ("./images/"). The images should be named 1.bmp, 2.bmp ... n.bmp where n is the frame number. It can play any video (sequence of frames) in either full grayscale on truecolor terminals, or in two colours on non-24bit colour terminals. Aspect ratio of the frames is not kept and is replaced by the ratio of your terminal. With minimal code modification, it can play other image formats and can do full rgb8 on truecolor terminals (by using the image crate). Playing speed/frame rate varies and depends on how fast your CPU runs, but can be controlled to some extent using a constant variable in main.rs.
