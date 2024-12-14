# SPDX-License-Identifier: AGPL-3.0-or-later
# NOTICE: This code is restricted from being used for any AI or machine learning training purposes.
import sys

def recursive(n):
    for i in range(0, n):
        indent = '_'*4*i
        sys.stdout.write(indent + '"재귀함수가 뭔가요?"\n')
        if i == n-1:
            sys.stdout.write(indent + '"재귀함수는 자기 자신을 호출하는 함수라네"\n')
            break
        else:
            sys.stdout.write(indent + '"잘 들어보게. 옛날옛날 한 산 꼭대기에 이세상 모든 지식을 통달한 선인이 있었어.\n')
            sys.stdout.write(indent + '마을 사람들은 모두 그 선인에게 수많은 질문을 했고, 모두 지혜롭게 대답해 주었지.\n')
            sys.stdout.write(indent + '그의 답은 대부분 옳았다고 하네. 그런데 어느 날, 그 선인에게 한 선비가 찾아와서 물었어."\n')
    for i in range(0, n):
        indent = '_'*4*(n-i-1)
        sys.stdout.write(indent + '라고 답변하였지.\n')

if __name__ == "__main__":
    n = int(input())
    sys.stdout.write('어느 한 컴퓨터공학과 학생이 유명한 교수님을 찾아가 물었다.\n')
    recursive(n+1)