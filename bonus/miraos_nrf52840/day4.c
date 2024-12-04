/*
 * MIT License
 *
 * Copyright (c) 2023 LumenRadio AB
 * Copyright (c) 2024 Sebastian Andersson
 *
 * SPDX-License-Identifier: MIT
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

#include <mira.h>
#include <stdio.h>

MIRA_IODEFS(MIRA_IODEF_NONE,    /* fd 0: stdin */
            MIRA_IODEF_UART(0), /* fd 1: stdout */
            MIRA_IODEF_NONE     /* fd 2: stderr */
                                /* More file descriptors can be added, for use with dprintf(); */
);

PROCESS(solve_it, "Solving process");

static uint8_t map[150 * 150];
static int width;
static int height;
static int x;

int uart_input(uint8_t c, void *storage)
{
    static int pos;
    static bool done;
    static bool last_lf;
    if (done) {
        return 0;
    }
    if (c == '\n') {
        printf(".\n");
        if (last_lf) {
            done = true;
            process_start(&solve_it, NULL);
            return 0;
        }
        last_lf = true;
        height++;
        if (!width) {
            width = x;
        } else if (x != width) {
            printf("Different widths\n");
            done = true;
        }
        x = 0;
        return 0;
    }
    if (pos > sizeof(map)) {
        printf("Map too large\n");
        done = true;
    }
    map[pos++] = c;
    x++;
    last_lf = false;
    return 0;
}


void mira_setup(void)
{
    mira_status_t uart_ret;
    mira_uart_config_t uart_config = {
        .baudrate = 115200,
#if MIRA_PLATFORM_MKW41Z
        .tx_pin = MIRA_GPIO_PIN('C', 7),
        .rx_pin = MIRA_GPIO_PIN('C', 6)
#else
        .tx_pin = MIRA_GPIO_PIN(0, 6),
        .rx_pin = MIRA_GPIO_PIN(0, 8)
#endif
    };

    MIRA_MEM_SET_BUFFER(7592);

    uart_ret = mira_uart_init(0, &uart_config);
    if (uart_ret != MIRA_SUCCESS) {
        /* Nowhere to send an error message */
    }

    mira_uart_set_input_callback(0, uart_input, 0);
}

static bool is_xmas(int x_, int y_, int dir) {
    const int d_x[] = {-1, 0, 1, 1, 1, 0, -1, -1};
    const int d_y[] = {-1, -1, -1, 0, 1, 1, 1, 0};

    for(int i = 0; i < 3; ++i) {
        x_ += d_x[dir];
        y_ += d_y[dir];
        if (x_ < 0 || x_ >= width || y_ < 0 || y_ >= height) {
            return false;
        }
        int pos = x_ + y_ * width;
        if (map[pos] != "MAS"[i]) {
            return false;
        }
    }
    return true;
}

static bool is_ms(
        int x1, int y1,
        int x2, int y2)
{
    if ((map[x1 + y1 * width] == 'M' &&
        map[x2 + y2 * width] == 'S') ||
        (map[x1 + y1 * width] == 'S' &&
        map[x2 + y2 * width] == 'M')) {
        return true;
    }
    return false;
}

PROCESS_THREAD(solve_it, ev, data)
{
    PROCESS_BEGIN();
    /* Pause once, so we don't run anything before finish of startup */
    PROCESS_PAUSE();

    printf("Solving. Height=%d, Width=%d\n", height, width);

    static clock_t before;
    before = clock_time();

    static int y;
    static int count;
    for (y = 0; y < height; ++y) {
        for (x = 0; x < width; ++x) {
            int pos = x + y * width;
            if (map[pos] == 'X') {
                // printf("Found X: %d,%d\n", x, y);
                for(int i = 0; i < 8; ++i) {
                    if (is_xmas(x, y, i)) {
                        count++;
                    }
                }
            }
        }
    }
    clock_t after = clock_time();
    printf("Day 4, part 1: %d, time=%lu\n", count, after - before);
    before = after;

    count = 0;
    for (y = 1; y < height - 1; ++y) {
        for (x = 1; x < width - 1; ++x) {
            int pos = x + y * width;
            if (map[pos] == 'A') {
                if (is_ms(x-1, y-1, x+1, y+1) &&
                    is_ms(x+1, y-1, x-1, y+1)) {
                    count++;
                }
            }
        }
    }

    after = clock_time();
    printf("Day 4, part 2: %d, time=%lu\n", count, after - before);

    PROCESS_END();
}
