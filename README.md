# Bit stuff
## Board

The board is encoded as 64 4-bits tiles, and 2 bytes of header info.

### Header
| Attribute                     | Bits |
|-------------------------------|------|
| can white castle left         | 1    |
| can white castle right        | 1    |
| can black castle left         | 1    |
| can black castle right        | 1    |
| counter for the 50-moves rule | 7    |
| empty                         | 5    |

### Tiles
| Attribute  | Bits |
|------------|------|
| color      | 1    |
| piece type | 3    |

## Moves

Each move can be encoded with 2 bytes. 

| Attribute             | Bits |
|-----------------------|------|
| from position index   | 6    |
| to position index     | 6    |
| is capture/check/mate | 2    |
| empty                 | 2    |
The from position index field is important for differentiating from duplicate moves (i.e. 2 rooks on the same file).