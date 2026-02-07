# Prototype for synthetic division calculator in python

co = [] # Array of coeffients -- testing [2,-5,-1,3] 
div = 0 # Root of divisor -- testing 3 -- -3 
sol = [] # Solution array -- should end with [2, -11, 32, -93]

def displayPopulate():
    print("Please select an option:")
    print("    1: Populate Dividend")
    print("    2: Populate Divisor")
    print("    3: Finished")
    print(f"Dividend = {co}")
    print(f"Divisor = {div}")

finished = False
while not finished:
    displayPopulate()
    option = int(input("Selection: "))

    if option == 1:
        value = int(input("Value: "))
        co.append(value)

    elif option == 2:
        value = int(input("Value: "))
        div = -value      

    elif option == 3:
        finished = True


# Add highest degree term to solution every time 
sol.append(co[0])

# sol index 1 -> last index of sol
for i in range(1, len(co)):
    sol.append(div * sol[i-1] + co[i])

print(f"Solution is: \n{sol}")





    




