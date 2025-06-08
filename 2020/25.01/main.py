import sys


def calc_loop_size(
    exp_pk: int | None, exp_loop_size: int | None, sbj_nb: int
) -> tuple[int, int]:
    divisor = 20201227

    pk = 1
    loop_size = 0
    while True:
        if pk == exp_pk:
            break
        if loop_size == exp_loop_size:
            break
        loop_size += 1
        pk = (pk * sbj_nb) % divisor

    return (pk, loop_size)


def calc_enc_key(card_pk: int, door_pk: int) -> int:
    _, card_loop_size = calc_loop_size(card_pk, None, 7)
    card_enc_key, _ = calc_loop_size(None, card_loop_size, door_pk)
    return card_enc_key


def test_example():
    card_pk, door_pk = 5764801, 17807724
    _, card_loop_size = calc_loop_size(card_pk, None, 7)
    assert card_loop_size == 8
    _, door_loop_size = calc_loop_size(door_pk, None, 7)
    assert door_loop_size == 11

    card_enc_key, _ = calc_loop_size(None, card_loop_size, door_pk)
    assert card_enc_key == 14897079
    door_enc_key, _ = calc_loop_size(None, door_loop_size, card_pk)
    assert door_enc_key == 14897079


def main():
    card_pk, door_pk = [int(x) for x in sys.stdin.readline().strip().split(" ")]
    print(calc_enc_key(card_pk, door_pk))


if __name__ == "__main__":
    main()
