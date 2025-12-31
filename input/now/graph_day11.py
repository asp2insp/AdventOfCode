#!/usr/bin/env python3
"""
Parse and visualize a directed graph from an adjacency list file.
Input format: "from: to1 to2 to3..." per line
"""

import networkx as nx
import matplotlib.pyplot as plt
from pathlib import Path


def parse_adjacency_list(file_path):
    """
    Parse the adjacency list file and return a directed graph.
    Format: "from: to1 to2 to3..." per line

    Args:
        file_path: Path to the input file

    Returns:
        NetworkX DiGraph object
    """
    G = nx.DiGraph()

    with open(file_path, 'r', encoding='utf-8') as f:
        for line_num, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue

            # Parse format: "from: to1 to2 to3..."
            if ':' in line:
                from_node, to_nodes = line.split(':', 1)
                from_node = from_node.strip()
                to_nodes = to_nodes.strip()

                if not from_node:
                    print(f"Warning: Line {line_num} has empty from_node")
                    continue

                # Add edges for each destination node
                destinations = to_nodes.split()
                if destinations:
                    for to_node in destinations:
                        G.add_edge(from_node, to_node)
                else:
                    # Node with no outgoing edges
                    G.add_node(from_node)
            else:
                print(f"Warning: Line {line_num} doesn't have colon: {line}")

    return G


def visualize_graph(G, output_file='graph_visualization.png'):
    """
    Visualize the directed graph and save to a file.

    Args:
        G: NetworkX DiGraph object
        output_file: Path to save the visualization
    """
    plt.figure(figsize=(20, 20))

    # Use spring layout for better visualization
    pos = nx.spring_layout(G, k=0.5, iterations=50, seed=42)

    # Draw the graph
    nx.draw(G, pos,
            node_color='lightblue',
            node_size=300,
            font_size=6,
            font_weight='bold',
            arrows=True,
            arrowsize=10,
            edge_color='gray',
            alpha=0.7,
            with_labels=True)

    plt.title(f"Directed Graph Visualization\n{G.number_of_nodes()} nodes, {G.number_of_edges()} edges",
              fontsize=16)
    plt.axis('off')
    plt.tight_layout()
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    print(f"Graph visualization saved to {output_file}")

    # Also display if running interactively
    plt.show()


def print_graph_stats(G):
    """Print statistics about the graph."""
    print(f"\n=== Graph Statistics ===")
    print(f"Number of nodes: {G.number_of_nodes()}")
    print(f"Number of edges: {G.number_of_edges()}")

    if G.number_of_nodes() > 0:
        print(f"Is strongly connected: {nx.is_strongly_connected(G)}")
        print(f"Is weakly connected: {nx.is_weakly_connected(G)}")
    else:
        print("Graph is empty - skipping connectivity checks")

    # Find nodes with special properties
    in_degrees = dict(G.in_degree())
    out_degrees = dict(G.out_degree())

    # Nodes with no incoming edges (sources)
    sources = [node for node, deg in in_degrees.items() if deg == 0]
    print(f"\nSource nodes (no incoming edges): {len(sources)}")
    if sources and len(sources) <= 10:
        print(f"  {sources}")

    # Nodes with no outgoing edges (sinks)
    sinks = [node for node, deg in out_degrees.items() if deg == 0]
    print(f"\nSink nodes (no outgoing edges): {len(sinks)}")
    if sinks and len(sinks) <= 10:
        print(f"  {sinks}")

    # Most connected nodes
    print(f"\nTop 5 nodes by out-degree:")
    top_out = sorted(out_degrees.items(), key=lambda x: x[1], reverse=True)[:5]
    for node, deg in top_out:
        print(f"  {node}: {deg} outgoing edges")

    print(f"\nTop 5 nodes by in-degree:")
    top_in = sorted(in_degrees.items(), key=lambda x: x[1], reverse=True)[:5]
    for node, deg in top_in:
        print(f"  {node}: {deg} incoming edges")


def main():
    # Path to the input file
    input_file = Path(__file__).parent / 'input' / 'now' / 'day11'

    print(f"Reading adjacency list from: {input_file}")

    # Parse the graph
    G = parse_adjacency_list(input_file)

    # Print statistics
    print_graph_stats(G)

    # Visualize the graph
    print("\nGenerating visualization...")
    visualize_graph(G, 'day11_graph.png')


if __name__ == '__main__':
    main()
