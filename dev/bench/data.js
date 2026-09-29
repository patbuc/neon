window.BENCHMARK_DATA = {
  "lastUpdate": 1790715337544,
  "repoUrl": "https://github.com/patbuc/neon",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f00be9e07340de82793e1da1c3cae9fa8203c765",
          "message": "Merge pull request #218 from patbuc/217-benchmark-suite\n\n217: Add a benchmark suite with Python comparison and charts",
          "timestamp": "2026-09-26T22:03:24+02:00",
          "tree_id": "50586130185f920ae2b7aa9b8473128b365f0e00",
          "url": "https://github.com/patbuc/neon/commit/f00be9e07340de82793e1da1c3cae9fa8203c765"
        },
        "date": 1790453089126,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 644.9124193999993,
            "range": "± 7.548",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.6720406000004,
            "range": "± 0.632",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.4363798535901777,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 647.3115889999974,
            "range": "± 2.057",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 522.9452938000009,
            "range": "± 7.565",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.237818939522879,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 525.6589615999985,
            "range": "± 3.544",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.8123932000012,
            "range": "± 4.328",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6912419617120868,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 455.3373532000023,
            "range": "± 9.526",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.8927094000053,
            "range": "± 1.274",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 2.4105607603719914,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 445.3571347999997,
            "range": "± 1.665",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.66449820000253,
            "range": "± 1.149",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.9883502991462834,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 684.4033981999985,
            "range": "± 4.598",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 436.878567399998,
            "range": "± 2.724",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.5665758159597958,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 679.5958187999986,
            "range": "± 1.851",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 411.47213539999825,
            "range": "± 9.363",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6516205116522733,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 413.2366422000018,
            "range": "± 5.874",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 221.34051640000507,
            "range": "± 3.861",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.8669724319844063,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 438.59816979999664,
            "range": "± 2.810",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.2269979999993,
            "range": "± 12.235",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.7740706854354065,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e82c4b5738d5d908bbc5ee93c749167071d5534e",
          "message": "Merge pull request #220 from patbuc/219-scientific-notation\n\n219: Add scientific-notation number literals",
          "timestamp": "2026-09-26T22:56:39+02:00",
          "tree_id": "03c63cf687e3eb3226b3c376a052dbd38fa19f46",
          "url": "https://github.com/patbuc/neon/commit/e82c4b5738d5d908bbc5ee93c749167071d5534e"
        },
        "date": 1790456278743,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 540.7657049999983,
            "range": "± 7.478",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 145.88661239999965,
            "range": "± 1.412",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.7067534580712467,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 546.3765146000071,
            "range": "± 0.537",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 463.5675267999943,
            "range": "± 4.894",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1786341428435314,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 431.4917561999948,
            "range": "± 1.513",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 263.96705860000225,
            "range": "± 1.032",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6346424379181652,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 352.34050840000464,
            "range": "± 1.18",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 162.5549584000055,
            "range": "± 4.89",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 2.167516216472378,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 353.0427473999964,
            "range": "± 0.796",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.7118552000029,
            "range": "± 0.854",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 4.428986711125795,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 550.9385255999916,
            "range": "± 3.445",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 399.94400999999584,
            "range": "± 1.157",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3775391350404207,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 628.3896284000036,
            "range": "± 9.086",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 366.1399648000099,
            "range": "± 20.509",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.7162552269956044,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 359.3527818000098,
            "range": "± 6.793",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 219.0363734000016,
            "range": "± 12.413",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.6406077959653032,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 359.5926345999942,
            "range": "± 3.336",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 213.39382880000244,
            "range": "± 4.817",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6851126230881428,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "00eb429e6032fb0b4c02769801a20217aab7e053",
          "message": "Merge pull request #221 from patbuc/216-skip-unreachable-if-jump\n\n216: Skip the unreachable Jump after an if-branch that exits",
          "timestamp": "2026-09-26T23:19:41+02:00",
          "tree_id": "2e4fd926aeaca4a983a6afa8504a4fe35980722d",
          "url": "https://github.com/patbuc/neon/commit/00eb429e6032fb0b4c02769801a20217aab7e053"
        },
        "date": 1790457663370,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 484.75303600000075,
            "range": "± 2.085",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 154.7294038000075,
            "range": "± 2.365",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.1329083166801244,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 536.4047056000004,
            "range": "± 5.832",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 562.0868852000001,
            "range": "± 143.102",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9543092353224688,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 423.05097359999877,
            "range": "± 17.044",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 281.2049000000002,
            "range": "± 6.301",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5044224819695478,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 379.67315279999525,
            "range": "± 12.97",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 166.3971599999968,
            "range": "± 5.465",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 2.281728563155781,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 357.7789803999906,
            "range": "± 7.512",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.04193000000282,
            "range": "± 1.244",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 4.063733954946002,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 531.3252369999987,
            "range": "± 4.27",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 369.09223420000217,
            "range": "± 6.575",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.4395459664753778,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 604.961011600011,
            "range": "± 7.071",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 380.199108599993,
            "range": "± 7.853",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5911689373172353,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 358.85565160000397,
            "range": "± 12.111",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 195.35535060001052,
            "range": "± 4.951",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.8369379210644698,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 363.9973980000036,
            "range": "± 4.192",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 213.1082546000016,
            "range": "± 3.216",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.708039881811321,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4e2762c7e43b377ad9f1eb537c92e492f524b4a5",
          "message": "Merge pull request #222 from patbuc/215-duplicate-struct-fields\n\n215: Reject duplicate struct field names",
          "timestamp": "2026-09-26T23:21:24+02:00",
          "tree_id": "e752c9ec51c27b6d1f28cb5230f6943f3333d439",
          "url": "https://github.com/patbuc/neon/commit/4e2762c7e43b377ad9f1eb537c92e492f524b4a5"
        },
        "date": 1790457777288,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 690.8713301999995,
            "range": "± 2.411",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.57457780000095,
            "range": "± 0.721",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.683182115097881,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 644.9392100000068,
            "range": "± 3.802",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 542.0087668000008,
            "range": "± 14.88",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1899054950858148,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 577.3573693999992,
            "range": "± 6.902",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 308.00320039999747,
            "range": "± 9.542",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.874517435696113,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 445.13397559999817,
            "range": "± 2.148",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 191.38649780000208,
            "range": "± 1.785",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 2.3258379285730015,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 435.4241602000002,
            "range": "± 3.951",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.96568299999967,
            "range": "± 1.746",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.8889072842078,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 699.6516738000025,
            "range": "± 5.385",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 440.46296439999537,
            "range": "± 6.91",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.5884460904745477,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 724.5238838000006,
            "range": "± 93.734",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 414.3376513999982,
            "range": "± 9.729",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.748631536023626,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 450.8281557999993,
            "range": "± 13.34",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 228.3332067999993,
            "range": "± 5.464",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.9744309735678829,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 443.8899847999977,
            "range": "± 3.104",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.03479260000518,
            "range": "± 4.361",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.849273515692775,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f3af28db6ca115aa424e5169d503abcf1307247e",
          "message": "Merge pull request #224 from patbuc/4-negate-not-in-place\n\n4: Negate and Not in place on the VM stack",
          "timestamp": "2026-09-27T00:55:50+02:00",
          "tree_id": "f71ef2967cbbd1dea6de6e54a4845025fc8a54c9",
          "url": "https://github.com/patbuc/neon/commit/f3af28db6ca115aa424e5169d503abcf1307247e"
        },
        "date": 1790463445212,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 523.7304619999975,
            "range": "± 1.422",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 141.67149240000185,
            "range": "± 0.295",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.6967949806110085,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 530.2964939999981,
            "range": "± 1.274",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 452.3093453999991,
            "range": "± 4.096",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1724199364729713,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 421.79705759999706,
            "range": "± 0.91",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 258.2861517999959,
            "range": "± 1.636",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6330610629354065,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 342.9121385999963,
            "range": "± 1.126",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 159.4429810000065,
            "range": "± 3.025",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 2.1506882049576226,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 340.57182020001164,
            "range": "± 0.711",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 78.92960239999525,
            "range": "± 2.792",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 4.314880727183698,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 537.0404157999985,
            "range": "± 1.246",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 392.3624820000043,
            "range": "± 4.905",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3687353924934968,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 594.7301409999966,
            "range": "± 10.392",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 351.5302034000001,
            "range": "± 9.471",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6918322671786568,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 354.54815200000667,
            "range": "± 2.516",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 203.75803100000098,
            "range": "± 1.303",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.740045043917827,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 350.4346874000021,
            "range": "± 2.211",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 209.88149620000058,
            "range": "± 3.432",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6696788127814057,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ac689b78bb8c3f89b6ff50d111e562b4545f41eb",
          "message": "Merge pull request #226 from patbuc/223-field-method-symbols",
          "timestamp": "2026-09-27T08:27:43+02:00",
          "tree_id": "0b626a75d4d36ec9d61af84a7869cf048c46a5bf",
          "url": "https://github.com/patbuc/neon/commit/ac689b78bb8c3f89b6ff50d111e562b4545f41eb"
        },
        "date": 1790490582863,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 637.8655112000047,
            "range": "± 5.2",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 192.54574480000315,
            "range": "± 4.153",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.3127998329049246,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 663.3236162000003,
            "range": "± 37.809",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 536.2549941999987,
            "range": "± 11.137",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.2369555964500925,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 510.122804599996,
            "range": "± 2.848",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 319.07955899999365,
            "range": "± 11.575",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5987323230567902,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 338.78726240000105,
            "range": "± 2.808",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.8324185999959,
            "range": "± 3.653",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.7753129415088194,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 332.0387891999985,
            "range": "± 4.845",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.83478059999743,
            "range": "± 1.327",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.9690118531873453,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 593.9592262000076,
            "range": "± 7.811",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 438.9801395999996,
            "range": "± 5.879",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3530435038387514,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 666.908386199998,
            "range": "± 11.554",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 420.4116691999957,
            "range": "± 13.309",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5863222528267653,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 395.60850959999243,
            "range": "± 3.785",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 222.54591380000193,
            "range": "± 4.219",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.777648948232403,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 399.1360203999875,
            "range": "± 3.303",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.01451120000115,
            "range": "± 10.699",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6158403749681636,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dd38298240d1ccc3cdcf45fc7f2618f1845b17e3",
          "message": "Merge pull request #243 from patbuc/240-vscode-textmate-grammar\n\n240: Add a VS Code TextMate grammar for Neon",
          "timestamp": "2026-09-28T00:01:23+02:00",
          "tree_id": "612d5cae4d15888e5466b2d89718b7e0f757ea33",
          "url": "https://github.com/patbuc/neon/commit/dd38298240d1ccc3cdcf45fc7f2618f1845b17e3"
        },
        "date": 1790546546281,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 368.3315824000019,
            "range": "± 6.729",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 103.9048539999996,
            "range": "± 10.087",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.5448929306036394,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 378.63547420000145,
            "range": "± 2.858",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 274.7522392000036,
            "range": "± 11.337",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.3780978648344224,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 286.4771293999979,
            "range": "± 4.212",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 172.0402197999988,
            "range": "± 1.758",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.665175327798552,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.43900279999752,
            "range": "± 2.009",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 111.16923539999846,
            "range": "± 5.773",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.7040595999277706,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 189.18331819999707,
            "range": "± 3.258",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 60.003015999998865,
            "range": "± 1.372",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.1528968177199737,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 336.029452599999,
            "range": "± 3.55",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 246.07216100000073,
            "range": "± 2.942",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.365572810977175,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 363.11926559999677,
            "range": "± 0.37",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 238.7393410000044,
            "range": "± 18.456",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5209862944205332,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 227.22036620000097,
            "range": "± 1.802",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 125.71690759999683,
            "range": "± 1.007",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.8073970362281382,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 237.31809080000517,
            "range": "± 2.518",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 137.73068220000084,
            "range": "± 16.395",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.7230589946210526,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "cad92f003df264eabf43fc446a43e955fdbba195",
          "message": "Merge pull request #244 from patbuc/241-grammar-tests\n\n241: Test the Neon grammar against the script corpus in CI",
          "timestamp": "2026-09-28T01:17:22+02:00",
          "tree_id": "2482cc4c2916acc9f1e22f158a1112a11898c732",
          "url": "https://github.com/patbuc/neon/commit/cad92f003df264eabf43fc446a43e955fdbba195"
        },
        "date": 1790551132810,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 635.5626388000019,
            "range": "± 3.936",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.40924039999618,
            "range": "± 0.766",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.409501789912422,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 644.2940631999988,
            "range": "± 3.662",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 538.5623639999977,
            "range": "± 10.666",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1963221091327532,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 516.3235455999995,
            "range": "± 3.175",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 322.05743280000263,
            "range": "± 16.747",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6032033203240368,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 340.1966053999985,
            "range": "± 9.241",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.54525199999648,
            "range": "± 5.661",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.7668397525585553,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 332.463468200001,
            "range": "± 2.332",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.62181440000529,
            "range": "± 1.561",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.0054060313802364,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 582.672098599997,
            "range": "± 4.302",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 431.96581999999637,
            "range": "± 1.95",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3488847302779694,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 666.7312226000007,
            "range": "± 5.928",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.30407159999606,
            "range": "± 7.104",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6409661364565247,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 390.89657839999745,
            "range": "± 7.692",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 216.55279940000014,
            "range": "± 2.42",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.8050867016406587,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 413.7772464000051,
            "range": "± 18.098",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 249.77654619999896,
            "range": "± 9.284",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6565896706277974,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2030ccff7b7f0b6824531c4f4f3e1f4a24c9299b",
          "message": "Merge pull request #245 from patbuc/242-release-vsix\n\n242: Release the Neon VS Code extension as a VSIX",
          "timestamp": "2026-09-28T01:32:24+02:00",
          "tree_id": "7e301b629aba31a418ff881fc14b5190472069f9",
          "url": "https://github.com/patbuc/neon/commit/2030ccff7b7f0b6824531c4f4f3e1f4a24c9299b"
        },
        "date": 1790552013080,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 400.8057974000053,
            "range": "± 1.081",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 104.71817900000246,
            "range": "± 0.665",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.8274710391974622,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 404.45094439999707,
            "range": "± 26.059",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 309.0559283999994,
            "range": "± 2.344",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.3086658667053024,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 317.85224840000694,
            "range": "± 14.003",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 185.80858879999766,
            "range": "± 3.489",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.7106434662293228,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 207.10342019999928,
            "range": "± 14.555",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 114.81513040000095,
            "range": "± 6.257",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.8037990243836153,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 193.50054360000115,
            "range": "± 7.846",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 56.0675534000012,
            "range": "± 0.789",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.451203626088614,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 378.35627759999966,
            "range": "± 17.15",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 278.82298579999656,
            "range": "± 1.751",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3569766370388117,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 420.4590044000014,
            "range": "± 4.602",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 265.0268823999994,
            "range": "± 17.899",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5864768154553153,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 259.477907400003,
            "range": "± 15.108",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 144.519715200002,
            "range": "± 4.199",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.7954498944376511,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 251.6884370000014,
            "range": "± 14.359",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 164.98069520000342,
            "range": "± 9.45",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.525562955683291,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7d6aede98e649f49922c708d3c607784cdc489de",
          "message": "Merge pull request #247 from patbuc/vscode-ci-paths\n\nRun the grammar tests when the script corpus changes",
          "timestamp": "2026-09-29T22:31:07+02:00",
          "tree_id": "c191263f56256f34a4a518666128c88cfbba2392",
          "url": "https://github.com/patbuc/neon/commit/7d6aede98e649f49922c708d3c607784cdc489de"
        },
        "date": 1790713947539,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 550.2965433999975,
            "range": "± 1.761",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 147.13058440000282,
            "range": "± 1.225",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.7401913792710184,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 549.7725698000011,
            "range": "± 0.358",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 464.3627079999959,
            "range": "± 6.409",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.183929200878047,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 425.5163478000014,
            "range": "± 0.762",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 271.73588280000445,
            "range": "± 12.638",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5659188746639627,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 274.51558879999993,
            "range": "± 0.203",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 166.09368420000123,
            "range": "± 4.185",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.6527756014457646,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 262.63276700000233,
            "range": "± 2.223",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 81.66925599999786,
            "range": "± 2.864",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.2158094718042887,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 502.455272200001,
            "range": "± 2.593",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 405.7135078000016,
            "range": "± 1.797",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.2384484680448173,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 643.0687476000003,
            "range": "± 3.537",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 412.8323787999989,
            "range": "± 23.597",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5576993971966087,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 359.95263479999835,
            "range": "± 2.031",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 225.79574140000034,
            "range": "± 5.298",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5941515662261194,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 337.91403279999486,
            "range": "± 1.642",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 217.0643584000004,
            "range": "± 7.474",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.556745820874452,
            "unit": "ratio"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2b3a94b4cac766db40e5f55f23d496be74072b48",
          "message": "Merge pull request #248 from patbuc/vscode-ci-token-json-paths\n\nRun the grammar tests when the token dump changes",
          "timestamp": "2026-09-29T22:54:04+02:00",
          "tree_id": "f23e4a0c58e52900cdb1071567ed25df0227223f",
          "url": "https://github.com/patbuc/neon/commit/2b3a94b4cac766db40e5f55f23d496be74072b48"
        },
        "date": 1790715336975,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 624.2924541999997,
            "range": "± 7.214",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 195.19776980000358,
            "range": "± 1.361",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.1982560806900584,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 693.5095410000031,
            "range": "± 11.034",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 648.2503472000019,
            "range": "± 11.824",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0698174617190563,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 546.6666376000006,
            "range": "± 6.578",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 366.265663599998,
            "range": "± 9.782",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.4925413215829595,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 364.6935858000006,
            "range": "± 3.879",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 221.06673440000293,
            "range": "± 4.614",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.6496990684275281,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 353.1627681999993,
            "range": "± 4.204",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 113.11634800000263,
            "range": "± 1.065",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.1221196090947974,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 590.6371381999975,
            "range": "± 5.501",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 455.92886059999955,
            "range": "± 13.04",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.295458983278055,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 731.2146940000019,
            "range": "± 8.323",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 494.3211127999973,
            "range": "± 18.598",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4792301503331742,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 430.87428359999933,
            "range": "± 4.338",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 237.19016919999945,
            "range": "± 4.027",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.8165773271854486,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 445.2004174000024,
            "range": "± 6.281",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 272.89208999999914,
            "range": "± 9.795",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6314156170668186,
            "unit": "ratio"
          }
        ]
      }
    ]
  }
}