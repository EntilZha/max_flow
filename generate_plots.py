from collections import namedtuple
import matplotlib.pyplot as plt
from scipy import stats
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
    slope, intercept, r_value, p_value, std_err = stats.linregress(x, y)
    print(slope, intercept, r_value, p_value, std_err)
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
    all_x = list()
    all_y = list()
    for k, v in ff_data:
        x = list(map(_[1], v))
        all_x.extend(x)
        y = list(map(_[2], v))
        all_y.extend(y)
        ratio = 1 - k / max_flow
        if ratio > .8:
            ratio = .8
        plt.scatter(x, y, color=str(ratio))
    x = np.array(all_x)
    y = np.array(all_y)
    slope, intercept, r_value, p_value, std_err = stats.linregress(x, y)
    print(slope, intercept, r_value, p_value, std_err)
    plt.show()


if __name__ == '__main__':
    plot('data/performance/scale-experiments.txt')
