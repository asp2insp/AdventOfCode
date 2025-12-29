import matplotlib.pyplot as plt
import matplotlib.patches as patches
import sys

# Read the data
x_coords = []
y_coords = []

with open('input/now/day9', 'r') as f:
    for line in f:
        line = line.strip()
        if line and ',' in line:
            # Extract coordinates from the line
            coords = line.split(',')
            if len(coords) == 2:
                try:
                    x = int(coords[0])
                    y = int(coords[1])
                    x_coords.append(x)
                    y_coords.append(y)
                except ValueError:
                    pass

# Read rectangles if provided
rectangles = []
if len(sys.argv) > 1:
    rect_file = sys.argv[1]
    try:
        with open(rect_file, 'r') as f:
            for line in f:
                line = line.strip().lstrip("Cand ")
                if line and ' ' in line:
                    parts = line.split()
                    if len(parts) == 2:
                        try:
                            x1, y1 = map(int, parts[0].split(','))
                            x2, y2 = map(int, parts[1].split(','))
                            rectangles.append((x1, y1, x2, y2))
                        except ValueError:
                            pass
        print(f"Loaded {len(rectangles)} rectangles from '{rect_file}'")
    except FileNotFoundError:
        print(f"Warning: Rectangle file '{rect_file}' not found")

# Read outside points
outside_x_coords = []
outside_y_coords = []
try:
    with open('day9_outside.txt', 'r') as f:
        for line in f:
            line = line.strip().lstrip('Out ')
            if line and ',' in line:
                coords = line.split(',')
                if len(coords) == 2:
                    try:
                        x = int(coords[0])
                        y = int(coords[1])
                        outside_x_coords.append(x)
                        outside_y_coords.append(y)
                    except ValueError:
                        pass
    print(f"Loaded {len(outside_x_coords)} outside points from 'day9_outside.txt'")
except FileNotFoundError:
    print("No 'day9_outside.txt' file found, skipping outside points")

# Create the plot
fig, ax = plt.subplots(figsize=(12, 8))

# Draw rectangles first (so they appear behind points)
for x1, y1, x2, y2 in rectangles:
    # Calculate bottom-left corner and width/height
    x_min, x_max = min(x1, x2), max(x1, x2)
    y_min, y_max = min(y1, y2), max(y1, y2)
    width = x_max - x_min
    height = y_max - y_min
    rect = patches.Rectangle((x_min, y_min), width, height,
                             linewidth=1, edgecolor='blue',
                             facecolor='blue', alpha=0.3)
    ax.add_patch(rect)

ax.plot(x_coords, y_coords, color='green', linewidth=1, alpha=0.6)  # Green lines between adjacent points
ax.scatter(x_coords, y_coords, color='red', s=10, alpha=0.8)  # Red points
ax.scatter(outside_x_coords, outside_y_coords, color='purple', s=10, alpha=0.8)  # Purple outside points
plt.xlabel('X Coordinate')
plt.ylabel('Y Coordinate')
title = f'Day 9 Points ({len(x_coords)} points)'
if outside_x_coords:
    title += f', {len(outside_x_coords)} outside'
if rectangles:
    title += f', {len(rectangles)} rectangles'
plt.title(title)
plt.grid(True, alpha=0.3)
plt.axis('equal')
plt.tight_layout()

# Save the plot
plt.savefig('day9_plot.png', dpi=150)
print(f"Plot saved as 'day9_plot.png'")
print(f"Total points plotted: {len(x_coords)}")
print(f"X range: {min(x_coords)} to {max(x_coords)}")
print(f"Y range: {min(y_coords)} to {max(y_coords)}")

# Show the plot
plt.show()
