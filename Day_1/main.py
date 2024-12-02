def findSmallestNumPosInList(list, excluding_list):
    smallest = -1
    smallest_index = -1
    for i in range(len(list)):
        if smallest == -1 and smallest_index == -1 and not numInList(excluding_list, i):
            smallest = list[i]
            smallest_index = i

        elif list[i] < smallest and not numInList(excluding_list, i):
            smallest = list[i]
            smallest_index = i
    return smallest_index

def numInList(list, num):
    for i in range(len(list)):
        if list[i] == num:
            return True
    return False

def getListsFromFile():
    # text file organized as such "val1   val2" new row "val1   val2"
    file = open("input.txt", "r")
    lines = file.readlines()
    list_1 = []
    list_2 = []
    for line in lines:
        values = line.split()
        list_1.append(int(values[0]))
        list_2.append(int(values[1]))

    return list_1, list_2

def numTimesInList(list, num):
    count = 0
    for i in range(len(list)):
        if list[i] == num:
            count += 1
    return count

def main():
    list_1, list_2 = getListsFromFile()

    print(list_1)

    pos_list_1 = []
    pos_list_2 = []

    # total difference
    # differences = []
    # total_difference = 0
    # for i in range(len(list_1)):
    #     print(str((i/1000) * 100) + "%")
    #     pos_list_1.append(findSmallestNumPosInList(list_1, pos_list_1))
    #     pos_list_2.append(findSmallestNumPosInList(list_2, pos_list_2))

    # for i in range(len(pos_list_1)):
    #     differences.append(abs(list_1[pos_list_1[i]] - list_2[pos_list_2[i]]))

    # for i in range(len(differences)):
    #     total_difference += differences[i]
    #     print(total_difference)

    # similarity score
    similarity_score = 0
    for i in range(len(list_1)):
        print(str((i/1000) * 100) + "%")
        timesInList = numTimesInList(list_2, list_1[i])
        similarity_score += timesInList * list_1[i]

    print(similarity_score)

main()