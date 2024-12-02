def getLists():
    # read a txt file with a 2d array of numbers each row has a differeing list of numbers and can be varying lengths
    # return a list of lists as ints
    file = open("input.txt", "r")
    lines = file.readlines()
    lists = []
    for line in lines:
        values = line.split()
        list = []
        for value in values:
            list.append(int(value))
        lists.append(list)
    return lists

def getListOfDifferences(list):
    listOfDifferences = []
    for i in range(len(list) - 1):
        listOfDifferences.append(list[i + 1] - list[i])
    
    return listOfDifferences

def checkSafety(list):
    safe = True
    # check for a change less than
    # print(list)
    for i in range(len(list)):
        if not (abs(list[i]) <= 3 and abs(list[i]) >= 1):
            # print("not safe")
            safe = False
            break

    if not safe:
        return safe            

    # check for strictly increasing or decreasing
    for i in range(len(list) - 1):
        if list[i] > 0 and list[i + 1] < 0:
            safe = False
            break
        elif list[i] < 0 and list[i + 1] > 0:
            safe = False
            break

    return safe
    
def numInList(list, num):
    for val in list:
        if val == num:
            return True
    return False


def main():
    lists = getLists()
    numSafe = 0

    #check list for safety but removing only a single value at a time ofr example if i have a list of [0, 1, 2, 3] try removing 0 on one attempt then try removing 1 on another attempt
    for list in lists:
        for i in range(len(list)):
            modified_list = list[:i] + list[i+1:]
            # print(getListOfDifferences(modified_list))
            if checkSafety(getListOfDifferences(modified_list)):
                numSafe += 1
                break

    print(numSafe)

main()