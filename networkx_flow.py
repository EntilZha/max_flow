import click
import time
import networkx as nx
from networkx.algorithms.flow import edmonds_karp
from networkx.drawing.nx_pydot import write_dot


def parse_dicaps_graph(input_file):
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
        return G, source, sink

@click.command()
@click.option('--ek', is_flag=True)
@click.option('--draw', is_flag=True)
@click.argument('input_file')
def cli(ek, draw, input_file):
    g, source, sink = parse_dicaps_graph(input_file)
    if draw:
        write_dot(g, 'graph.dot')
        return
    if ek:
        print('Using Edmonds Karp')
        flow_func = edmonds_karp
    else:
        print('Using Preflow Push')
        flow_func = None
    t0 = time.time()
    flow = nx.maximum_flow_value(g, source, sink, flow_func=flow_func)
    t1 = time.time()
    print("Max Flow Solution: {0}".format(flow))
    print("Runtime: {0}s".format(t1 - t0))


if __name__ == '__main__':
    cli()
