import click
from functional import seq
from collections import namedtuple

Edge = namedtuple('Edge', 'u v cap')

@click.command()
@click.argument('source_file')
@click.argument('destination_file')
def cli(source_file, destination_file):
    with open(source_file, 'r') as f:
        meta = f.readline().split()
        n_vertexes = meta[2]
        source = int(f.readline().split()[1])
        sink = int(f.readline().split()[1])
        edges = seq(f.readlines())\
            .map(str.split)\
            .filter(lambda e: len(e) == 4)\
            .map(lambda e: Edge(int(e[1]), int(e[2]), int(e[3])))\
            .filter(lambda e: e.cap != 0).cache()
        vertex_map = edges.flat_map(lambda e: (e.u, e.v)).distinct().zip_with_index().to_dict()

        source = vertex_map[source]
        sink = vertex_map[sink]
        new_edges = edges.map(
            lambda e: "a {0} {1} {2}".format(vertex_map[e.u], vertex_map[e.v], e.cap))
    with open(destination_file, 'w') as o:
        print('p max {0} {1}'.format(n_vertexes, new_edges.len()), file=o)
        print('n {0} s'.format(source), file=o)
        print('n {0} t'.format(sink), file=o)
        for e in new_edges:
            print(e, file=o)


if __name__ == '__main__':
    cli()
