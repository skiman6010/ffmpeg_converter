# Project Scope

I just needed a way to quickly and effectively use ffmpeg to convert a bunch of videos to a format that would be compatible with label-studio. This will take an input directory and an output directory and convert all the videos to x264 and with pixel format of yuv420p in the input directory to the output directory. It will also create a file called `video_list.txt` in the output directory that contains the list of videos that were converted. This is useful for the label-studio video classification task. Should work well and is async for that sweet, sweet speed.
