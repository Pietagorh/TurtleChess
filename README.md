# Bit stuff
## Board

The board is encoded as 64 4-bits tiles (32 bytes), and 2 bytes of header info.

### Header
| Attribute               | Bits |
|-------------------------|------|
| white to move           | 1    |
| white O-O               | 1    |
| white O-O-O             | 1    |
| black O-O               | 1    |
| black O-O-O             | 1    |
| can en passant          | 1    |
| en passant column       | 3    |
| counter (50-moves rule) | 7    |

*The counter needs to count up to 100 because it counts half-moves*

### Tiles
| Attribute  | Bits |
|------------|------|
| color      | 1    |
| piece type | 3    |

## Moves

Each move can be encoded with 2 bytes.

| Attribute           | Bits |
|---------------------|------|
| from position index | 6    |
| to position index   | 6    |
| is capture          | 1    |
| is check            | 1    |
| is mate             | 1    |
| *empty*             | 1    |

*The `from position index` field is important for differentiating duplicate moves (i.e. 2 rooks on the same file).*

## 3-fold repetition

To implement this rule, we need to keep track of every position (without header) since the last capture or pawn move, so at most 100 * 32 = 320 bytes.