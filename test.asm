#loads 15 into register D
lval 15
#moves (copies) value from D into A
move d:a
#loads 3 into register D
lval 3
#moves (copies) value from D into B
move d:b
#add A & B, output goes into C
addn

#counting to 100
#load 100 into the ram
#load 100 into register D
lval 100
#you can only move values from register C into ram
move d:c
#save value to ram at address 0
save 0

#save 1 to ram
lval 1
move d:c
save 1

.loop
#load 1 from ram and move to reg a
load 1
move d:a
#put 1 into reg b
lval 1
move d:b
addn
#save the sum to ram
save 1
#move sum to reg a
move c:a
#load 100(from ram address 0) to reg b
load 0
move d:b
#check if reg a & reg b are equal
equl
#if not then go back to the loop label
jmpf loop

#load the last sum output to reg c
load 1
move d:c