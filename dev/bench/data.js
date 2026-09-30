window.BENCHMARK_DATA = {
  "lastUpdate": 1790784705445,
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
          "id": "6e8bc37bdac0e8bfaed3c9fad24f5736a2ef6f11",
          "message": "Merge pull request #233 from patbuc/228-opcode-histogram\n\n228: Document VM profiling and add an opcode histogram",
          "timestamp": "2026-09-29T23:23:52+02:00",
          "tree_id": "1b78ef3e7466811061d3b0a2a8b07c72b3ff7b9c",
          "url": "https://github.com/patbuc/neon/commit/6e8bc37bdac0e8bfaed3c9fad24f5736a2ef6f11"
        },
        "date": 1790717120634,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 543.6947471999929,
            "range": "± 9.371",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 143.98247120000178,
            "range": "± 1.106",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.7761176250736983,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 536.7946976000042,
            "range": "± 5.938",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 462.84447959999966,
            "range": "± 6.973",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1597733607278062,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 418.720636799992,
            "range": "± 1.313",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 261.3807202000089,
            "range": "± 2.92",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6019568561888817,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 273.2536132000007,
            "range": "± 0.543",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 171.89270979999378,
            "range": "± 9.41",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5896754057687825,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 260.17951620000304,
            "range": "± 1.302",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.09247380000579,
            "range": "± 0.432",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.289560987280486,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 491.74168820000546,
            "range": "± 10.425",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 389.2346325999881,
            "range": "± 1.455",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.2633554340098063,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 591.4816350000024,
            "range": "± 5.976",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 358.01122780000014,
            "range": "± 10.018",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6521315228985736,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 353.71746499999404,
            "range": "± 2.131",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 208.51574939999864,
            "range": "± 10.253",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.6963585053781858,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 336.6897800000004,
            "range": "± 8.12",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 210.6729143999928,
            "range": "± 6.314",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.5981635843359838,
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
          "id": "6f19543853c833e06c36ae95170cc9cdd108012b",
          "message": "Merge pull request #237 from patbuc/235-opcode-pair-counts\n\n235: Count executed opcode pairs in opcode-stats",
          "timestamp": "2026-09-29T23:28:34+02:00",
          "tree_id": "37cfcbffabac4e3223f3540d420263a339a9f343",
          "url": "https://github.com/patbuc/neon/commit/6f19543853c833e06c36ae95170cc9cdd108012b"
        },
        "date": 1790717400929,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 659.4149146000035,
            "range": "± 20.996",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.9638379999999,
            "range": "± 1.391",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.4712654868554713,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 643.5093486,
            "range": "± 9.445",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.5599375999994,
            "range": "± 7.108",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.2106054333316647,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 505.44135380000057,
            "range": "± 2.344",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 312.35079260000305,
            "range": "± 6.122",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6181849567043347,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 328.92554120000455,
            "range": "± 1.821",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 197.55831499999772,
            "range": "± 7.082",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.6649541741637568,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 325.82170279999616,
            "range": "± 7.522",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.06591339999738,
            "range": "± 0.956",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.933588648630371,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 607.6361682000027,
            "range": "± 16.79",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.8898994000036,
            "range": "± 1.707",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.3719801897112258,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 666.4290617999967,
            "range": "± 3.174",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 410.3288332000034,
            "range": "± 13.732",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6241341282374968,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 411.537572200001,
            "range": "± 17.006",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 229.72659820000274,
            "range": "± 10.392",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.7914232632379445,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 405.93610619999936,
            "range": "± 5.76",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 251.46491500000252,
            "range": "± 17.709",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.6142852620215242,
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
          "id": "39abb193bb31f0d7983a7d344d5a0d31f486ab07",
          "message": "Merge pull request #238 from patbuc/236-feature-ci\n\n236: Check cargo feature builds in CI",
          "timestamp": "2026-09-29T23:32:33+02:00",
          "tree_id": "d74d24c91c378c3ef8e9311a19bc1c377b7a7c9e",
          "url": "https://github.com/patbuc/neon/commit/39abb193bb31f0d7983a7d344d5a0d31f486ab07"
        },
        "date": 1790717616886,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 387.38419160000035,
            "range": "± 4.569",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 106.66770339999943,
            "range": "± 0.88",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.6316914984784647,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 386.83544360000184,
            "range": "± 1.721",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 289.4066145999986,
            "range": "± 5.752",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.3366503185653302,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 297.064521800003,
            "range": "± 1.787",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 179.99833219999886,
            "range": "± 12.404",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.6503737460741033,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 193.99549540000294,
            "range": "± 0.951",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 112.32503660000077,
            "range": "± 5.382",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.7270904267838147,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 194.05054119999932,
            "range": "± 2.054",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 59.38677140000266,
            "range": "± 0.228",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.2675718282942494,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 352.3962179999984,
            "range": "± 3.722",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 264.6373014000005,
            "range": "± 2.945",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.331619602133676,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 384.97016379999707,
            "range": "± 2.224",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 250.96503080000048,
            "range": "± 18.331",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5339593829978178,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 251.86109939999994,
            "range": "± 7.344",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 141.23368440000093,
            "range": "± 1.89",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.7832934152357092,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 256.1721294000023,
            "range": "± 10.69",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 142.7087297999975,
            "range": "± 11.507",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.7950697883655802,
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
          "id": "cbc9b18110b1241b0d5b5a50ec02bfca09eba898",
          "message": "Merge pull request #249 from patbuc/229-cache-frame-ip-chunk\n\n229: Cache the frame ip and chunk in the VM",
          "timestamp": "2026-09-30T00:18:18+02:00",
          "tree_id": "c339dc17c51da86f405ad0d0e7d2dbe43eb853fb",
          "url": "https://github.com/patbuc/neon/commit/cbc9b18110b1241b0d5b5a50ec02bfca09eba898"
        },
        "date": 1790720381460,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 535.5226420000008,
            "range": "± 37.312",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 196.8351015999957,
            "range": "± 1.722",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.7206663732583585,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 631.9131682000034,
            "range": "± 10.729",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 620.5453567999882,
            "range": "± 39.589",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0183190660850907,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 496.18472640000846,
            "range": "± 3.333",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 358.28382040000406,
            "range": "± 12.134",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.3848929204953928,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 372.0673149999982,
            "range": "± 92.511",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 221.6172849999964,
            "range": "± 2.673",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.6788731754384782,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 335.53330920000235,
            "range": "± 2.579",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 113.16917639999247,
            "range": "± 1.599",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.9648824872072206,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 539.7606409999923,
            "range": "± 4.281",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 459.9703637999994,
            "range": "± 12.038",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.1734683003070316,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 693.2422478000035,
            "range": "± 4.386",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 472.9798721999998,
            "range": "± 24.252",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4656908011232783,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 393.5847153999987,
            "range": "± 0.834",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 233.09051180000324,
            "range": "± 7.244",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.6885488489454388,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 395.26097559999585,
            "range": "± 1.858",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 263.84597360000157,
            "range": "± 11.769",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.4980746918625463,
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
          "id": "429abddc7574315225f1e6540f1a1ce828544cbe",
          "message": "Merge pull request #250 from patbuc/232-native-method-dispatch\n\n232: Dispatch native methods by symbol id",
          "timestamp": "2026-09-30T00:28:42+02:00",
          "tree_id": "a26bd679f371c81a6f1ee204cddf232d045dcf2a",
          "url": "https://github.com/patbuc/neon/commit/429abddc7574315225f1e6540f1a1ce828544cbe"
        },
        "date": 1790721007065,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 514.1733573999971,
            "range": "± 8.366",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 195.50414860000274,
            "range": "± 1.444",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.6299869393154656,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 641.3014153999995,
            "range": "± 3.813",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 634.0244265999985,
            "range": "± 14.971",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0114774581147046,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 504.6458374000025,
            "range": "± 1.865",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 365.11185960000034,
            "range": "± 15.457",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.3821677497763811,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 335.9957668000021,
            "range": "± 3.901",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 223.60496240000316,
            "range": "± 9.833",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5026310829316254,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 333.6844444000036,
            "range": "± 7.07",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.1336236000019,
            "range": "± 0.456",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.9757750948128456,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 519.0379949999965,
            "range": "± 3.699",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 469.4803075999971,
            "range": "± 2.883",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.1055586072466816,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 645.8372588000003,
            "range": "± 6.981",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 497.15456700000686,
            "range": "± 18.228",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2990673357326141,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 363.6223550000011,
            "range": "± 7.008",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 234.3104903999972,
            "range": "± 5.527",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5518825229687856,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 331.3947572000018,
            "range": "± 5.905",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 276.51289399999825,
            "range": "± 6.208",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1984784955453247,
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
          "id": "5c74bddea9a8bc45b1c2879600a052a17b05eb11",
          "message": "Merge pull request #251 from patbuc/230-binary-ops-in-place\n\n230: Run number operators in place on the stack",
          "timestamp": "2026-09-30T00:59:21+02:00",
          "tree_id": "2d9c75ff0ff3db8d40271e157da085ad460160f9",
          "url": "https://github.com/patbuc/neon/commit/5c74bddea9a8bc45b1c2879600a052a17b05eb11"
        },
        "date": 1790722817371,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 315.8353158000011,
            "range": "± 5.799",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 102.38217279999162,
            "range": "± 1.252",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.084866311804109,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 343.8664497999923,
            "range": "± 1.235",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 272.53441760000214,
            "range": "± 1.038",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.2617358674480665,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 265.8905221999987,
            "range": "± 1.423",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 172.7772305999963,
            "range": "± 4.373",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5389210793381267,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 174.34250160000602,
            "range": "± 2.87",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 108.37192800000253,
            "range": "± 1.17",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.6087422713380346,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 184.1776024000012,
            "range": "± 20.535",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 56.93459000000303,
            "range": "± 1.011",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 3.23489819457717,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 302.78093839999656,
            "range": "± 6.779",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 258.7099178000017,
            "range": "± 1.55",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.170349173215943,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 343.14154960000565,
            "range": "± 3.272",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 266.1995674000025,
            "range": "± 35.501",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2890387199029025,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 212.2286994000035,
            "range": "± 2.118",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 149.46523399999592,
            "range": "± 8.856",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4199201628387328,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 188.27262739999924,
            "range": "± 0.755",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 134.30795279999188,
            "range": "± 3.792",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.401798057933101,
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
          "id": "050cc1b0e7677640407fdb7cebba4dff23a7d04d",
          "message": "Merge pull request #246 from patbuc/claude/festive-darwin-03fpgw\n\nAdd fibers",
          "timestamp": "2026-09-30T12:26:25+02:00",
          "tree_id": "be4c3bb8a8c4fd639323bb70c50bda88a1914875",
          "url": "https://github.com/patbuc/neon/commit/050cc1b0e7677640407fdb7cebba4dff23a7d04d"
        },
        "date": 1790764077184,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 519.6187037999977,
            "range": "± 6.338",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 196.01973960000123,
            "range": "± 1.547",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.650848862774402,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 603.6042047999985,
            "range": "± 0.682",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 629.7890388000013,
            "range": "± 26.929",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9584228489433614,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 497.1756051999989,
            "range": "± 2.975",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 363.13820280000755,
            "range": "± 14.934",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.3691085139665415,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 327.51513179999563,
            "range": "± 15.1",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 215.0231275999971,
            "range": "± 7.691",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5231623474906615,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 312.10182520000274,
            "range": "± 1.504",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 115.57477519999964,
            "range": "± 4.767",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.700432033373349,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 518.6888880000026,
            "range": "± 4.397",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 477.48759380000365,
            "range": "± 22.133",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0862876747688992,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 689.397723600004,
            "range": "± 15.049",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 488.79253979999646,
            "range": "± 5.675",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4104096676313655,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 369.4859865999973,
            "range": "± 4.235",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 245.99209739999992,
            "range": "± 4.667",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5020238068834704,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 337.9168163999992,
            "range": "± 10.673",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 276.9127590000039,
            "range": "± 5.548",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2203006377181573,
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
          "id": "117d16156ff6ee2f81c2d35a6580901ff7f2a719",
          "message": "Merge pull request #272 from patbuc/231-cut-value-drop-clone\n\n231: Cut Value clone and drop cost on stack reads",
          "timestamp": "2026-09-30T14:19:08+02:00",
          "tree_id": "ad23935bc9600729f03eb7dd603b42d2d30c27f7",
          "url": "https://github.com/patbuc/neon/commit/117d16156ff6ee2f81c2d35a6580901ff7f2a719"
        },
        "date": 1790770927953,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 541.1482462000038,
            "range": "± 2.286",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.44820399999844,
            "range": "± 0.948",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.902405250307524,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 594.7123287999972,
            "range": "± 4.116",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 529.4314780000036,
            "range": "± 11.782",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1233036823700029,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 494.6984441999973,
            "range": "± 1.914",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 326.37945440000067,
            "range": "± 38.095",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.51571564181154,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 307.0304519999979,
            "range": "± 3.951",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 196.7833885999994,
            "range": "± 16.847",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.560245781843391,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 300.62877500000127,
            "range": "± 10.717",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.12829060000138,
            "range": "± 2.088",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.7052407031265675,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 536.1474870000023,
            "range": "± 3.217",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.060049599998,
            "range": "± 4.048",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.2267135545577552,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 654.0923436000014,
            "range": "± 4.676",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 407.9609998000052,
            "range": "± 13.67",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6033207682122979,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 352.1187280000021,
            "range": "± 2.648",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 221.12317299999802,
            "range": "± 4.341",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5924098918389038,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 322.3519218000007,
            "range": "± 2.451",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 256.36643599999616,
            "range": "± 9.412",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.257387382020654,
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
          "id": "f0624883f2bfc7feeb0f393373bcfcd2537a940b",
          "message": "Merge pull request #273 from patbuc/271-trim-rc-churn\n\n271: Trim refcount churn on the call path",
          "timestamp": "2026-09-30T15:06:40+02:00",
          "tree_id": "d3295f08e06d2b63389f436efa8735e386739e5b",
          "url": "https://github.com/patbuc/neon/commit/f0624883f2bfc7feeb0f393373bcfcd2537a940b"
        },
        "date": 1790773678323,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 434.6894851999991,
            "range": "± 1.911",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 144.2479301999981,
            "range": "± 0.426",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 3.0134885443230077,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 484.4211243999979,
            "range": "± 1.452",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 463.878685200001,
            "range": "± 10.089",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.044284076538546,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 375.9342196000034,
            "range": "± 1.253",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 265.1233086000019,
            "range": "± 12.899",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.4179598979250243,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 243.27837220000106,
            "range": "± 0.473",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 164.0580822000004,
            "range": "± 5.167",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.4828795322831128,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 233.97015419999718,
            "range": "± 1.124",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 78.87260959999765,
            "range": "± 1.348",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.96643100040149,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 424.04129720000014,
            "range": "± 1.374",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 397.50010699999905,
            "range": "± 1.802",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0667702718379424,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 564.3793926000001,
            "range": "± 12.127",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 362.96191580000254,
            "range": "± 11.714",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5549273023756618,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 315.7774044000007,
            "range": "± 2.474",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 213.50452919999725,
            "range": "± 4.989",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4790196984730044,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 275.7251683999982,
            "range": "± 1.239",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 216.90518399999803,
            "range": "± 4.244",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2711783246268595,
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
          "id": "7cda135348dfe64675b8ef7361647997208c69f9",
          "message": "Merge pull request #286 from patbuc/writing-neon-skill\n\nAdd writing-neon skill and fix README drift",
          "timestamp": "2026-09-30T17:22:16+02:00",
          "tree_id": "418f53d70212a979a746c10fb05ee7171d371f5c",
          "url": "https://github.com/patbuc/neon/commit/7cda135348dfe64675b8ef7361647997208c69f9"
        },
        "date": 1790781806546,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 426.6270462000051,
            "range": "± 5.133",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 152.47487320000062,
            "range": "± 1.012",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.7980154188447846,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 473.56381300000123,
            "range": "± 1.981",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 502.18341979999934,
            "range": "± 7.005",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9430096540993006,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 389.88691919999496,
            "range": "± 3.047",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 277.72885120000126,
            "range": "± 4.466",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.403840175463892,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 257.4210063999999,
            "range": "± 17.312",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 168.00838420000161,
            "range": "± 6.02",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5321914297655475,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 236.35037879999743,
            "range": "± 2.531",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.49175959999604,
            "range": "± 2.507",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.701401593482275,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.27776559999654,
            "range": "± 1.946",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 366.08230160000517,
            "range": "± 5.668",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0715562153250815,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 525.3480710000019,
            "range": "± 10.376",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 372.32475520000037,
            "range": "± 8.197",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4109942024075262,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 288.2482097999997,
            "range": "± 1.421",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 197.28858939999725,
            "range": "± 19.278",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.461048561787749,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 262.80054400000097,
            "range": "± 2.152",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 207.40590240000074,
            "range": "± 7.617",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2670832457466266,
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
          "id": "b994f8475e5f105f0faaacf7520ad34459e949a1",
          "message": "Merge pull request #289 from patbuc/278-profile-release-build\n\n278: Profile with line tables instead of full debug info",
          "timestamp": "2026-09-30T17:24:31+02:00",
          "tree_id": "0eb2f6a5eedc92826b01cbd61baf9bbd938ad992",
          "url": "https://github.com/patbuc/neon/commit/b994f8475e5f105f0faaacf7520ad34459e949a1"
        },
        "date": 1790781958875,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 542.703868199996,
            "range": "± 2.449",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.52132959999415,
            "range": "± 1.969",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.894091404735928,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 596.1267990000039,
            "range": "± 0.561",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 539.7030464000011,
            "range": "± 14.167",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1045459220146485,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 495.5975058000007,
            "range": "± 1.425",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 315.8838987999957,
            "range": "± 9.384",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.568922973544125,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 302.51263840000036,
            "range": "± 3.159",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.2031552000011,
            "range": "± 3.574",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5904711889868723,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 292.84642299999746,
            "range": "± 1.974",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.59965380000085,
            "range": "± 1.368",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.6719648543268955,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 505.2938485999988,
            "range": "± 2.194",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 431.6592492000012,
            "range": "± 1.869",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.1705850147690924,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 648.162795199994,
            "range": "± 2.951",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 404.71163139999646,
            "range": "± 7.756",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.6015422955793004,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 341.70434779999823,
            "range": "± 1.2",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 217.7828747999996,
            "range": "± 2.462",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5690138543436973,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 324.74047839999685,
            "range": "± 2.677",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 238.45829119999848,
            "range": "± 4.116",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.3618334542523087,
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
          "id": "c3c8d8254595887e82ddd44cde7cf7d1caa79c90",
          "message": "Merge pull request #290 from patbuc/279-drop-self-copy-setlocal\n\n279: Drop the self-copy SetLocal after declarations",
          "timestamp": "2026-09-30T17:40:52+02:00",
          "tree_id": "dbcf53df90e0fd9551f63db771ae905e4ea3f815",
          "url": "https://github.com/patbuc/neon/commit/c3c8d8254595887e82ddd44cde7cf7d1caa79c90"
        },
        "date": 1790782945615,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 556.3957246000001,
            "range": "± 18.239",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 197.46556160000068,
            "range": "± 1.391",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.817684866625362,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 591.0169926000037,
            "range": "± 3.873",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 643.8105769999993,
            "range": "± 13.029",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9179982648840582,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 504.6801992000013,
            "range": "± 2.345",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 357.50941059999946,
            "range": "± 10.917",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.4116557053785204,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 318.3902599999982,
            "range": "± 1.35",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 219.38452619999964,
            "range": "± 2.008",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.4512885913828808,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 286.7084999999946,
            "range": "± 0.823",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 113.21264719999817,
            "range": "± 1.423",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.5324776612060287,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 508.33752620000325,
            "range": "± 9.496",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 470.5062489999989,
            "range": "± 11.221",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.080405472361759,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 694.2894121999984,
            "range": "± 10.823",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 469.8157615999975,
            "range": "± 23.207",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4777908042836556,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 376.19048279999845,
            "range": "± 12.927",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 240.21655539999927,
            "range": "± 2.145",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5660472783550687,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 327.27493120000304,
            "range": "± 2.454",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 280.33080799999937,
            "range": "± 15.04",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1674597363554982,
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
          "id": "a14915c25d6a4801e0d0a16b79f9b555c3fce34c",
          "message": "Merge pull request #291 from patbuc/287-method-docs-sync\n\n287: Test native-method docs against the registry",
          "timestamp": "2026-09-30T17:52:23+02:00",
          "tree_id": "d48c41a99840a78676012f215e26ae901c5afd50",
          "url": "https://github.com/patbuc/neon/commit/a14915c25d6a4801e0d0a16b79f9b555c3fce34c"
        },
        "date": 1790783631536,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 558.6501788000021,
            "range": "± 34.002",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.9617702000029,
            "range": "± 3.588",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.9408558270004668,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 573.7317182000027,
            "range": "± 0.939",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 533.2555821999961,
            "range": "± 9.666",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.075903820515143,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 491.6837311999956,
            "range": "± 3.22",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 317.7109839999986,
            "range": "± 7.607",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.547581783322914,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 308.5266170000011,
            "range": "± 18.388",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 204.79328019999343,
            "range": "± 23.128",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5065270535180921,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 281.84102100000246,
            "range": "± 1",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.86201600000629,
            "range": "± 2.741",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.519541762951837,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 508.8418694000012,
            "range": "± 2.676",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 445.8618132000055,
            "range": "± 4.428",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.1412546541000677,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 655.5510994000002,
            "range": "± 14.588",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 409.8044275999996,
            "range": "± 9.51",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5996681715695569,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 371.2014487999937,
            "range": "± 5.454",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 239.37236019999943,
            "range": "± 2.198",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5507281145151803,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 316.74716179999507,
            "range": "± 0.958",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.83958719999646,
            "range": "± 10.048",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2780329622821434,
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
          "id": "a267dc2fa071ac77aadc29d5f34415ec1cd77123",
          "message": "Merge pull request #292 from patbuc/280-scalar-clone-drop\n\n280: Skip drop glue and clone dispatch for scalar values",
          "timestamp": "2026-09-30T17:54:06+02:00",
          "tree_id": "cb0ab29557865f2efcfe3455fb3beab5b44d690c",
          "url": "https://github.com/patbuc/neon/commit/a267dc2fa071ac77aadc29d5f34415ec1cd77123"
        },
        "date": 1790783734467,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 528.1253494000794,
            "range": "± 3.909",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 199.90968920005798,
            "range": "± 6.244",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.6418196712394577,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 558.1819042000006,
            "range": "± 0.894",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 622.0492119999562,
            "range": "± 49.025",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.897327564173556,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 486.6209682000317,
            "range": "± 1.697",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 351.644974599958,
            "range": "± 9.176",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.3838416680165173,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 309.8388555999918,
            "range": "± 1.219",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 219.1678175999641,
            "range": "± 6.017",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.4137059856366547,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 275.9366864000185,
            "range": "± 2.805",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.77481340005215,
            "range": "± 3.055",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.468683936983336,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 500.01142060004895,
            "range": "± 35.699",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 478.90141899997616,
            "range": "± 3.734",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0440800564845973,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 664.6144660000573,
            "range": "± 9.401",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 478.9991015999476,
            "range": "± 11.174",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3875067067560654,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 367.41652780006007,
            "range": "± 2.177",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 251.85259320001023,
            "range": "± 6.87",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4588554484657383,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 325.1463036000132,
            "range": "± 3.309",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 278.3213566000086,
            "range": "± 7.34",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.168240581937445,
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
          "id": "8be622c5ef4b8fab5aa9977c99040b26ccda683e",
          "message": "Merge pull request #298 from patbuc/288-check-mode\n\n288: Add a compile-only --check mode and edit hook",
          "timestamp": "2026-09-30T18:05:34+02:00",
          "tree_id": "71b6cf46aeda9b3ee044d7b55d66ab2e8a76ff58",
          "url": "https://github.com/patbuc/neon/commit/8be622c5ef4b8fab5aa9977c99040b26ccda683e"
        },
        "date": 1790784415627,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 519.8315438000066,
            "range": "± 2.013",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 192.45424359999106,
            "range": "± 1.033",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.701065635530786,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 557.5737703999948,
            "range": "± 1.945",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 535.4242667999983,
            "range": "± 5.061",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0413681354645627,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 480.1438660000031,
            "range": "± 0.993",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.49436639998953,
            "range": "± 13.276",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5414207054500977,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 302.2200152000039,
            "range": "± 17.021",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.37177159999987,
            "range": "± 4.645",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5710205956225862,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 271.51937980000866,
            "range": "± 0.434",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.94855039999061,
            "range": "± 0.516",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.4472548656213147,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 492.18464560000257,
            "range": "± 3.124",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 440.8209566000039,
            "range": "± 2.903",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.1165182558382893,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 614.5039596000004,
            "range": "± 1.234",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 420.2700697999944,
            "range": "± 19.023",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.462164459849452,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 351.36574020000353,
            "range": "± 2.37",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 225.62829799999804,
            "range": "± 0.939",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5572769165683578,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 313.45312579999813,
            "range": "± 1.846",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.8262289999975,
            "range": "± 16.363",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.26481013355532,
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
          "id": "0b2700ff43e075ca8674e9e7d96639858bb4175e",
          "message": "Merge pull request #300 from patbuc/281-inline-closure-call\n\n281: Inline the closure call path",
          "timestamp": "2026-09-30T18:10:18+02:00",
          "tree_id": "5217fcfa5de00661466e63e4315afdc99aa8c923",
          "url": "https://github.com/patbuc/neon/commit/0b2700ff43e075ca8674e9e7d96639858bb4175e"
        },
        "date": 1790784704108,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 515.9839333999912,
            "range": "± 1.288",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.00022180000633,
            "range": "± 0.481",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.7300705178324502,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 633.7632887999916,
            "range": "± 160.615",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 536.5110285999947,
            "range": "± 4.26",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.1812679609844612,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 475.66372500000966,
            "range": "± 3.219",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.17373500000076,
            "range": "± 3.323",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5335396628602629,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 296.59706580000034,
            "range": "± 0.593",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 189.40048299999717,
            "range": "± 3.821",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5659784024943841,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 271.522585799994,
            "range": "± 0.467",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.86501219999718,
            "range": "± 0.693",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.4491278214101966,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 493.43078420000097,
            "range": "± 2.748",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 468.17813559999877,
            "range": "± 8.148",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.053938120300384,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 621.9848220000017,
            "range": "± 4.281",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.66452099999833,
            "range": "± 4.598",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5035972156771071,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 369.8859957999957,
            "range": "± 14.767",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 237.6278538000065,
            "range": "± 6.67",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5565767644028,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 314.388581999998,
            "range": "± 2.73",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 253.31212720000167,
            "range": "± 13.945",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2411114520062976,
            "unit": "ratio"
          }
        ]
      }
    ]
  }
}