#!/usr/bin/env python3
"""
Stream input collapsing identical consecutive lines.
Displays repeated lines as "line contents... [x N]" where N is the count.
Updates in place as new lines arrive.
"""
import sys

def clear_line():
    """Clear the current line and move cursor to beginning"""
    sys.stdout.write('\r\033[K')
    sys.stdout.flush()

def main():
    prev_line = None
    count = 0

    try:
        for line in sys.stdin:
            line = line.rstrip('\n')

            if line == prev_line:
                # Same line as before, increment count
                count += 1
                # Update the current line display in place
                clear_line()
                sys.stdout.write(f"{line} [x {count}]")
                sys.stdout.flush()
            else:
                # Different line
                if prev_line is not None:
                    # Finalize previous line with newline
                    sys.stdout.write('\n')

                # Start new line
                prev_line = line
                count = 1
                sys.stdout.write(line)
                sys.stdout.flush()

        # Final newline after all input
        if prev_line is not None:
            sys.stdout.write('\n')

    except KeyboardInterrupt:
        # Clean exit on Ctrl+C
        if prev_line is not None:
            sys.stdout.write('\n')
        sys.exit(0)
    except BrokenPipeError:
        # Handle pipe being closed
        sys.exit(0)

if __name__ == '__main__':
    main()
