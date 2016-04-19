from collections import namedtuple
import itertools as it
import random
import click

Edge = namedtuple('Edge', 'u v capacity')

@click.command()
@click.argument('filename')
@click.option('--flow', default=100)
@click.option('--layer-size', default=500)
@click.option('--n-layers', default=1000)
@click.option('--connect-ratio', default=1)
def cli(filename, flow, layer_size, n_layers, connect_ratio):
    generate_file(filename, flow, layer_size, n_layers, connect_ratio)

def generate_file(filename, flow, layer_size, n_layers, connect_ratio):
    edges = generate_network(flow, layer_size, n_layers, connect_ratio)
    with open(filename, 'w') as f:
        n = layer_size * n_layers
        f.write('p max {0} {1}\n'.format(layer_size * n_layers + 2, len(edges)))
        f.write('n {0} s\n'.format(n))
        f.write('n {0} t\n'.format(n + 1))
        for _, v in edges.items():
            f.write('a {0} {1} {2}\n'.format(v.u, v.v, v.capacity))


def generate_network(flow, layer_size, n_layers, connect_ratio):
    edges = dict()
    n = layer_size * n_layers
    for i in range(n_layers - 1):
        for k, v in rand_from_layer(flow, i, layer_size, connect_ratio).items():
            edges[k] = v
    for i in range(layer_size):
        edges[(n, i)] = Edge(n, i, random.randint(1, flow))
        edges[(i + (n_layers - 1) * layer_size, n + 1)] = Edge(
            i + (n_layers - 1) * layer_size,
            n + 1,
            random.randint(1, flow))
    return edges

def rand_from_layer(flow, layer, layer_size, connect_ratio):
    left = [i for i in range(layer *  layer_size, (layer + 1) * layer_size)]
    right = [i for i in range((layer + 1) * layer_size, (layer + 2) * layer_size)]
    random.shuffle(left)
    random.shuffle(right)
    edges = {(u, v): Edge(u, v, flow) for u, v in zip(left, right)}
    if connect_ratio > 1:
        all_edges = list(it.product(left, right))
        random.shuffle(all_edges)
        i = 0
        while i < int(connect_ratio * layer_size):
            e = all_edges.pop()
            if e not in edges:
                edges[e] = Edge(e[0], e[1], random.randint(1, flow))
                i += 1
    return edges


if __name__ == '__main__':
    cli()
