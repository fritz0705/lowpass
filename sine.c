#include <unistd.h>
#include <stdint.h>
#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

#define INT int32_t

struct st {
	INT y0;
	INT y1;
	// Inverse of the square of the frequency
	INT f2inv;
	// Dampening factor (rp/rq)
	INT rp;
	INT rq;
};

extern INT n(struct st *, INT, INT);
inline INT n(struct st *st, INT x, INT xp) {
	INT k1_0, k2_0, k3_0, k4_0, k1_1, k2_1, k3_1, k4_1;
	INT xi = x / 2 + xp / 2;

	k1_0 = st->y1;
	k1_1 = x;
	k1_1 -= st->y0 / st->f2inv;
	k1_1 -= (st->y1) / st->rq * st->rp;

	k2_0 = st->y1 + k1_1 / 2;
	k2_1 = xi;
	k2_1 -= (st->y0 + k1_0 / 2) / st->f2inv;
	k2_1 -= (st->y1 + k1_1 / 2) / st->rq * st->rp;

	k3_0 = st->y1 + k2_1 / 2;
	k3_1 = xi;
	k3_1 -= (st->y0 + k2_0 / 2) / st->f2inv;
	k3_1 -= (st->y1 + k2_1 / 2) / st->rq * st->rp;

	k4_0 = st->y1 + k3_1;
	k4_1 = xp;
	k4_1 -= (st->y0 + k3_0) / st->f2inv;
	k4_1 -= (st->y1 + k3_1) / st->rq * st->rp;

	st->y0 += (k1_0 + 2 * k2_0 + 2 * k3_0 + k4_0) / 6;
	st->y1 += (k1_1 + 2 * k2_1 + 2 * k3_1 + k4_1) / 6;

	fprintf(stderr, "%d\t%d\n", st->y0, st->y1);

	return st->y0;
}

int main(int argc, char **argv) {
	static struct st ST = {
		.y0 = 0,
		.y1 = 0,
		.f2inv = 301,
		.rp = 0,
		.rq = 1,
	};
	if (argc > 1)
		ST.y0 = (INT)atoi(argv[1]);
	if (argc > 2)
		ST.y1 = (INT)atoi(argv[2]);
	if (argc > 3)
		ST.f2inv = (INT)atoi(argv[3]);
	if (argc > 4)
		ST.rp = (INT)atoi(argv[4]);
	if (argc > 5)
		ST.rq = (INT)atoi(argv[5]);
#define BUFSIZE 4096UL
	static INT xbuf[BUFSIZE], ybuf[BUFSIZE - 1];
	ssize_t len;
	while ((len = read(0, xbuf + 1, BUFSIZE - 1)) > 0) {
		size_t samples = (size_t)len/sizeof *xbuf;
		for (size_t j = 0; j + 1 < samples; ++j)
			ybuf[j] = n(&ST, xbuf[j], xbuf[j+1]);
		xbuf[0] = xbuf[samples - 1];
		if (!write(1, ybuf, (samples - 1) * sizeof *ybuf))
			break;
	}
}
