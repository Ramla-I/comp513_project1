# comp513_project1
Testbench for project 1 of comp 513

I ran a quick demo for matrix inversion, for 1000 matrix inversions 
of different sizes the time is:
8x8 = 0.25 s
16x16 = 1.8 s
32x32 = 12.7 s
64x64 = 98.2 s

The core svd algorithm which decomposes the input matrix into 3 
matrices S,U,V takes the time:
8x8 = 0.08 s
16x16 = 0.37 s
32x32 = 2.2 s
64x64 = 15.2 s

The matrix mult takes
8x8 = 0.2 s
16x16 = 1.35 s
32x32 = 10.72 s
64x64 = 83.5 s
