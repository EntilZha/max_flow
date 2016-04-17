import click
import time
import networkx as nx
from networkx.algorithms.flow import shortest_augmenting_path


@click.command()
@click.argument('input_file')
def cli(input_file):
    with open(input_file, 'r') as f:
        f.readline()
        source = int(f.readline().split()[1])
        sink = int(f.readline().split()[1])
        G = nx.DiGraph()
        for line in f.readlines():
            _, u, v, c = line.split()
            u = int(u)
            v = int(v)
            G.add_edge(u, v, capacity=int(c))
        t0 = time.time()
        print("Max Flow Solution: {0}".format(
            nx.maximum_flow_value(G, source, sink, flow_func=shortest_augmenting_path)))
        print("Runtime: {0}s".format(time.time() - t0))


if __name__ == '__main__':
    cli()
