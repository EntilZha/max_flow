from collections import namedtuple
import matplotlib.pyplot as plt
from fn import _
from functional import seq
import numpy as np

Result = namedtuple('Result', 'algorithm vertexes edges flow runtime')
plt.style.use('ggplot')

def parse_line(line):
    tokens = list(map(lambda x: x.split(':')[1], line.strip().split('\t')))
    return Result(tokens[0], int(tokens[1]), int(tokens[2]),
                  int(tokens[3]), float(tokens[4].replace('s', '')))

def plot(filename):
    data = seq.open(filename).map(parse_line)
    bfs = data.filter(_.algorithm == 'bfs')
    dfs = data.filter(_.algorithm == 'dfs')
    x = np.array(bfs.map(lambda x: x.vertexes * x.edges * x.edges).list())
    y = np.array(bfs.map(_.runtime).list())
    plt.title('Numerical Performance of Edmonds-Karp')
    plt.xlabel('Input Size in VE^2')
    plt.ylabel('Running Time in Seconds')
    plt.scatter(x, y)
    plt.show()
    plt.clf()
    ff_data = dfs.map(lambda x: (x.flow, x.flow * x.edges, x.runtime)).group_by(_[0]).cache()
    plt.title('Numerical Performance of Ford-Fulkerson')
    plt.xlabel('Input Size in Ef')
    plt.ylabel('Running Time in Seconds')
    max_flow = ff_data.max_by(lambda kv: kv[0])[0]
    for k, v in ff_data:
        x = list(map(_[0], v))
        y = list(map(_[1], v))
        plt.scatter(x, y, color=str(k / max_flow))
    plt.show()


if __name__ == '__main__':
    plot('data/performance/scale-experiments.txt')
