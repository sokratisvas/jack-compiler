expected = [
        "../examples/ArrayTest/MainT.xml", 
        "../examples/ExpressionLessSquare/MainT.xml",
        "../examples/ExpressionLessSquare/SquareT.xml",
        "../examples/ExpressionLessSquare/SquareGameT.xml",
        "../examples/Square/MainT.xml",
        "../examples/Square/SquareT.xml",
        "../examples/Square/SquareGameT.xml",
]

actual = [
        "../examples/ArrayTest/MainT-actual.xml",
        "../examples/ExpressionLessSquare/MainT-actual.xml",
        "../examples/ExpressionLessSquare/SquareT-actual.xml",
        "../examples/ExpressionLessSquare/SquareGameT-actual.xml",
        "../examples/Square/MainT-actual.xml",
        "../examples/Square/SquareT-actual.xml",
        "../examples/Square/SquareGameT-actual.xml",
]

for i in range(len(expected)):
    with open(expected[i], 'r') as file1:
        with open(actual[i], 'r') as file2:
            diff = set(file1).difference(file2)

    diff.discard('\n')

    if len(diff) == 0:
        print(f"Test {i}:\tComparison passed. All lines match!")
    else:
        print(f"Test {i}:\tComparison failed. Found differences:")
        for line in diff:
            print(line)
        break


  
