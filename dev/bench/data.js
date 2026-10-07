window.BENCHMARK_DATA = {
  "lastUpdate": 1791395970299,
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
          "id": "bf2e41107996df177e8805d2a58465e532f7c92c",
          "message": "Merge pull request #299 from patbuc/fix-opcode-stats-pairs\n\nFix Features on main and run it on pull requests",
          "timestamp": "2026-09-30T18:12:33+02:00",
          "tree_id": "0d4d81826471037159be0d9f753dc968e132de1a",
          "url": "https://github.com/patbuc/neon/commit/bf2e41107996df177e8805d2a58465e532f7c92c"
        },
        "date": 1790784842142,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 516.4037487999963,
            "range": "± 4.065",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.85729239999773,
            "range": "± 2.893",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.76362641333019,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 563.7151657999937,
            "range": "± 6.231",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 551.9454838000001,
            "range": "± 64.59",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0213239936650307,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 477.487095999993,
            "range": "± 3.569",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 303.5974440000075,
            "range": "± 4.208",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5727638866418823,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 295.8930605999967,
            "range": "± 0.381",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 189.78466160000949,
            "range": "± 1.785",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5590989182446235,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 273.16683580000927,
            "range": "± 2.25",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.46016219999433,
            "range": "± 3.327",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.429009797391374,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 487.9559438000001,
            "range": "± 2.594",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 443.8958392000018,
            "range": "± 11.529",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0992577553315308,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 616.7510131999961,
            "range": "± 1.526",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 416.91516599999545,
            "range": "± 9.595",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4793201674990226,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 337.9199244000006,
            "range": "± 3.234",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 220.66050580000365,
            "range": "± 5.569",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5314019297421324,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 311.7112316000089,
            "range": "± 3.811",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 241.1481092000031,
            "range": "± 6.063",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.292613210338225,
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
          "id": "26cb4508e71d03400c0524864b0ef676ef08a72b",
          "message": "Merge pull request #301 from patbuc/282-struct-field-slots\n\n282: Look up struct fields by slot in constant time",
          "timestamp": "2026-09-30T18:38:03+02:00",
          "tree_id": "1b790d9518d53b9d6984679f0e9bd17b3e176493",
          "url": "https://github.com/patbuc/neon/commit/26cb4508e71d03400c0524864b0ef676ef08a72b"
        },
        "date": 1790786356071,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 289.67482240000066,
            "range": "± 3.214",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 108.40824200000156,
            "range": "± 6.34",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.672073793060831,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 328.6244878000048,
            "range": "± 0.201",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 312.9246766000051,
            "range": "± 5.302",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0501712149089089,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 258.1287824000043,
            "range": "± 0.955",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 187.02632819999394,
            "range": "± 4.875",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.3801735022246597,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 171.4993177999986,
            "range": "± 1.279",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 113.99693840000396,
            "range": "± 2.699",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.5044203836266594,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 168.59548540000162,
            "range": "± 10.58",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 59.08271380000372,
            "range": "± 2.308",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 2.8535501258574723,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 301.54260319999366,
            "range": "± 1.568",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 280.58657219999645,
            "range": "± 2.801",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 1.0746865070401879,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 367.4205996000012,
            "range": "± 3.555",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 266.50357919999976,
            "range": "± 17.207",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3786704129938436,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 219.55820560000348,
            "range": "± 4.916",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 142.53870560000053,
            "range": "± 1.886",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5403409528366214,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 196.2576965999972,
            "range": "± 8.406",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 146.95766820000244,
            "range": "± 2.804",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.335470948905502,
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
          "id": "cfbda56cd916f1a19c9e70e42190aff5582e614f",
          "message": "Merge pull request #302 from patbuc/283-get-local-field\n\n283: Fuse a local's field read into GetLocalField",
          "timestamp": "2026-09-30T18:58:17+02:00",
          "tree_id": "e8a9376ab8a1311b853fdf8b03c0de31a13e9537",
          "url": "https://github.com/patbuc/neon/commit/cfbda56cd916f1a19c9e70e42190aff5582e614f"
        },
        "date": 1790787576334,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 509.0246445999952,
            "range": "± 4.65",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.47847779999915,
            "range": "± 1.183",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.7151097585879667,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 558.2001426000005,
            "range": "± 0.882",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 550.6664978000032,
            "range": "± 22.489",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0136809572220125,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 480.8798423999974,
            "range": "± 2.139",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.4745596000015,
            "range": "± 9.004",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.5488542540153267,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 266.72878819999823,
            "range": "± 0.429",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 191.70246020000263,
            "range": "± 3.3",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.3913686236562683,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 220.08704080000143,
            "range": "± 9.517",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.26736240000048,
            "range": "± 2.027",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.9959400135248042,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 435.1049377999999,
            "range": "± 0.866",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 435.3243533999972,
            "range": "± 5.201",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.9994959721451749,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 620.9558770000029,
            "range": "± 6.779",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 412.00971279999976,
            "range": "± 20.581",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.5071389283034478,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 337.24205720000384,
            "range": "± 1.084",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 220.9701569999993,
            "range": "± 1.398",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5261882499364152,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 309.11009460000685,
            "range": "± 1.112",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 236.48845360000053,
            "range": "± 2.139",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.3070832418856249,
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
          "id": "62038f7cd9f88d37a91617c57e93c395ba95d37d",
          "message": "Merge pull request #307 from patbuc/284-store-assign\n\n284: Store without pushing in assignment statements",
          "timestamp": "2026-09-30T22:34:43+02:00",
          "tree_id": "6ba6e93219042fee25c1663967a084edddb85c66",
          "url": "https://github.com/patbuc/neon/commit/62038f7cd9f88d37a91617c57e93c395ba95d37d"
        },
        "date": 1790800605138,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 515.0733460000026,
            "range": "± 2.579",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.41702019999593,
            "range": "± 0.784",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.7482741185958406,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 537.6501253999947,
            "range": "± 0.968",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 537.996800999997,
            "range": "± 12.391",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9993556177297748,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 460.42462240000077,
            "range": "± 1.371",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.2092744000165,
            "range": "± 8.233",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.4794694768902952,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 242.2393437999972,
            "range": "± 0.934",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.0112566000057,
            "range": "± 4.661",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.26158928434402,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 203.46786979999933,
            "range": "± 37.457",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.59230240000397,
            "range": "± 1.934",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.823314560449396,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 439.90439020000167,
            "range": "± 6.046",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.6219500000059,
            "range": "± 1.602",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.9938603139767376,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 587.6467233999961,
            "range": "± 2.401",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.10864919999517,
            "range": "± 15.658",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4224992009680801,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 346.2259442000061,
            "range": "± 4.909",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 226.8692834000035,
            "range": "± 2.998",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.5261032212525636,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 322.5412332000019,
            "range": "± 34.278",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.66264000000047,
            "range": "± 4.989",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.3129437719956167,
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
          "id": "e422f8dcf833bf9c1311f424045243197c1df223",
          "message": "Merge pull request #323 from patbuc/285-number-literal-ops\n\n285: Apply number literals directly in arithmetic and comparisons",
          "timestamp": "2026-09-30T23:03:51+02:00",
          "tree_id": "ececddb598d1b933e8295405c56da55211b1a2bd",
          "url": "https://github.com/patbuc/neon/commit/e422f8dcf833bf9c1311f424045243197c1df223"
        },
        "date": 1790802297258,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 287.7993830000037,
            "range": "± 0.654",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 153.98552220000852,
            "range": "± 1.656",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8690028704528934,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 412.7766680000036,
            "range": "± 0.567",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 496.5048990000014,
            "range": "± 13.39",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8313647434926971,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 306.0672323999995,
            "range": "± 3.507",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 281.5002435999986,
            "range": "± 7.845",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0872716431283438,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 164.8415218000025,
            "range": "± 0.898",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 169.69745199999693,
            "range": "± 3.615",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9713847783642945,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 147.34231339998587,
            "range": "± 0.606",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.59936500000458,
            "range": "± 2.175",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6630177135014257,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 293.61282219999225,
            "range": "± 2.226",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 367.65418460000205,
            "range": "± 2.915",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7986113976084216,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 461.00549520000413,
            "range": "± 4.082",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 384.2596753999999,
            "range": "± 15.302",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1997238448716085,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 268.45422359999134,
            "range": "± 4.316",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 191.9881738000015,
            "range": "± 6.193",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.398285208336042,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 247.38570700000082,
            "range": "± 5.485",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 220.41032659998905,
            "range": "± 10.712",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1223870987177835,
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
          "id": "b848bb6518e85fac0f57c159d6bca803fe677190",
          "message": "Merge pull request #276 from patbuc/worktree-bridge-cse_019XCspURmRq3evZnfuyjcaq\n\n275: Deduplicate postfix resolvers and inline MapKey conversions",
          "timestamp": "2026-09-30T23:19:22+02:00",
          "tree_id": "2000afca61b1dfd38c906dfa0b79007a9d6e99a7",
          "url": "https://github.com/patbuc/neon/commit/b848bb6518e85fac0f57c159d6bca803fe677190"
        },
        "date": 1790803241856,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 379.1580785999997,
            "range": "± 0.402",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.0463127999917,
            "range": "± 1.736",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0163015852551003,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 515.8299310000018,
            "range": "± 0.863",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 532.1272862000001,
            "range": "± 15.662",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9693732014451277,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 376.929249799997,
            "range": "± 1.406",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 312.6405613999964,
            "range": "± 13.146",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2056313106403007,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 198.71103780000112,
            "range": "± 1.637",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 197.19407320000073,
            "range": "± 14.53",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0076927494593706,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.87007099999914,
            "range": "± 0.266",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.0970985999918,
            "range": "± 2.089",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6045916731694276,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.09011399999554,
            "range": "± 1.079",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.5972171999945,
            "range": "± 1.779",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8858847248983571,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 559.458176600009,
            "range": "± 8.77",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 426.47196039999926,
            "range": "± 15.572",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3118287450252966,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 337.90015360000325,
            "range": "± 2.444",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 236.98594799999455,
            "range": "± 8.416",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4258235834304025,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 291.43194220000055,
            "range": "± 2.087",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.61160100000052,
            "range": "± 4.211",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2112131792016128,
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
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "distinct": true,
          "id": "1de640e921364707ce6cfcf5c26291683e591a9c",
          "message": "Merge pull request #276 from patbuc/worktree-bridge-cse_019XCspURmRq3evZnfuyjcaq\n\n275: Deduplicate postfix resolvers and inline MapKey conversions",
          "timestamp": "2026-10-01T09:22:25+02:00",
          "tree_id": "d0d3e56ae2396b458a7d5d8ef48bbc041501f5c7",
          "url": "https://github.com/patbuc/neon/commit/1de640e921364707ce6cfcf5c26291683e591a9c"
        },
        "date": 1790839531293,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 370.59186839999967,
            "range": "± 1.414",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.90662940000504,
            "range": "± 4.854",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9514425039865924,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 519.2240838000004,
            "range": "± 0.599",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 534.789791999998,
            "range": "± 23.84",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9708937821311338,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 375.50138339999535,
            "range": "± 2.126",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 320.88858220000134,
            "range": "± 24.027",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1701924101679482,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 197.06030720000456,
            "range": "± 0.777",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.2512860000012,
            "range": "± 2.615",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.025014247238915,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 180.69818359999488,
            "range": "± 2.541",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 114.01645420000364,
            "range": "± 4.603",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.584843037505977,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.7400765999977,
            "range": "± 3.838",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 444.39079740000125,
            "range": "± 2.046",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8837718487822056,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 567.9634740000012,
            "range": "± 25.388",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 420.701839000003,
            "range": "± 25.216",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3500380111245416,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 331.7180172000036,
            "range": "± 6.474",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 239.04618799999753,
            "range": "± 8.503",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.3876733194340125,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 294.6219886000051,
            "range": "± 7.84",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.02566459999457,
            "range": "± 6.322",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.227460359670271,
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
          "id": "4c8edfcb80e13f93a377bb7e980e848daba23749",
          "message": "Merge pull request #330 from patbuc/329-compound-assignment\n\n329: Add compound assignment for variables",
          "timestamp": "2026-10-01T15:03:01+02:00",
          "tree_id": "b5f527ef684bd224e38d8f5fa4d4578c49d5f59c",
          "url": "https://github.com/patbuc/neon/commit/4c8edfcb80e13f93a377bb7e980e848daba23749"
        },
        "date": 1790859890796,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 360.16832440000144,
            "range": "± 0.738",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 198.8165543999969,
            "range": "± 1.616",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.811561041719809,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 527.8796494000005,
            "range": "± 0.499",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 621.6086399999938,
            "range": "± 31.822",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.84921543143288,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 392.09576840000295,
            "range": "± 1.708",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 361.9520657999942,
            "range": "± 10.517",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0832809243218007,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 217.4014199999931,
            "range": "± 11.499",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 223.52847839999868,
            "range": "± 6.66",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9725893611236356,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 188.8670675999947,
            "range": "± 0.531",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 116.97624220000762,
            "range": "± 4.839",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6145762938517647,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 368.7680586000056,
            "range": "± 6.372",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 463.272958400006,
            "range": "± 8.276",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7960060087979458,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 635.0488946000041,
            "range": "± 22.28",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 509.8783045999937,
            "range": "± 24.117",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2454911081149225,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 338.99822340000014,
            "range": "± 5.411",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 250.08180980000816,
            "range": "± 20.06",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.3555493047299159,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 295.62787879999064,
            "range": "± 3.329",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 269.33850620000044,
            "range": "± 16.252",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.097607182021232,
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
          "id": "f55412a2a9b8a8808d62c7e00b05ef4d1698dded",
          "message": "Merge pull request #332 from patbuc/331-power-assign\n\n331: Add **= compound assignment",
          "timestamp": "2026-10-01T15:35:47+02:00",
          "tree_id": "9b8a185b4b4a386ba007665d3960f19d60585fb5",
          "url": "https://github.com/patbuc/neon/commit/f55412a2a9b8a8808d62c7e00b05ef4d1698dded"
        },
        "date": 1790861839715,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 372.1887797999983,
            "range": "± 2.871",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.7904911999991,
            "range": "± 1.102",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.981936238739654,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 522.9417246000025,
            "range": "± 9.121",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 539.3975945999955,
            "range": "± 17.556",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9694921331412384,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 375.38648400000056,
            "range": "± 0.348",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 321.9461064000029,
            "range": "± 15.915",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1659916878559808,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 196.83685219999631,
            "range": "± 0.22",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.62645400000133,
            "range": "± 4.133",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0435272891256013,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.75327320000076,
            "range": "± 0.224",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.26341199999865,
            "range": "± 3.271",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6011741492410985,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 383.55040040000006,
            "range": "± 2.197",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 450.45017699999903,
            "range": "± 16.366",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8514824057889112,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 555.388028400003,
            "range": "± 3.324",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 421.9148949999976,
            "range": "± 16.941",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3163508446413255,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 341.70581379999874,
            "range": "± 7.624",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 240.34413299999926,
            "range": "± 8.403",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4217356152396687,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 293.8269761999976,
            "range": "± 4.294",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 253.87605719999726,
            "range": "± 13.67",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1573638705462013,
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
          "id": "169b1833d94dc14dad7203a7825cb2a805abbc65",
          "message": "Merge pull request #332 from patbuc/331-power-assign\n\n331: Add **= compound assignment",
          "timestamp": "2026-10-01T15:35:47+02:00",
          "tree_id": "9b8a185b4b4a386ba007665d3960f19d60585fb5",
          "url": "https://github.com/patbuc/neon/commit/169b1833d94dc14dad7203a7825cb2a805abbc65"
        },
        "date": 1790953595160,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 370.5084050000039,
            "range": "± 1.328",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.37802160000194,
            "range": "± 1.58",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9668345694102993,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 519.3838851999999,
            "range": "± 0.793",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.1178205999994,
            "range": "± 19.496",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9779070952905633,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 377.07705300000214,
            "range": "± 1.38",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.7909611999977,
            "range": "± 9.594",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2093905851174653,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 196.99189819999674,
            "range": "± 0.75",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 189.86518760000024,
            "range": "± 4.264",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0375356361536414,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.57912779999958,
            "range": "± 0.103",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.788767599999,
            "range": "± 1.002",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6209145718487186,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 395.60171359999856,
            "range": "± 24.299",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 444.1336248000056,
            "range": "± 8.09",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8907267802075084,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 556.4780216000003,
            "range": "± 5.351",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 409.8415019999976,
            "range": "± 9.789",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3577883618043238,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 336.45194520000246,
            "range": "± 1.876",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.56537399999502,
            "range": "± 2.687",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4466983601781134,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 286.5426798000044,
            "range": "± 2.242",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.32595740000147,
            "range": "± 15.844",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1680079957164886,
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
            "email": "patricbucher@icloud.com",
            "name": "Patric Bucher",
            "username": "patbuc"
          },
          "distinct": true,
          "id": "d8eeabc2acb300766fdc0764d25c81851d812243",
          "message": "Merge pull request #332 from patbuc/331-power-assign\n\n331: Add **= compound assignment",
          "timestamp": "2026-10-01T15:35:47+02:00",
          "tree_id": "9b8a185b4b4a386ba007665d3960f19d60585fb5",
          "url": "https://github.com/patbuc/neon/commit/d8eeabc2acb300766fdc0764d25c81851d812243"
        },
        "date": 1790954279980,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 363.67666439999766,
            "range": "± 3.308",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 200.6129708000003,
            "range": "± 6.194",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8128272710868858,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 529.967688399995,
            "range": "± 11.285",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 644.0143437999978,
            "range": "± 10.386",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8229128644448009,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 394.99701200000175,
            "range": "± 1.044",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 359.20697140000186,
            "range": "± 15.03",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0996362639079886,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 212.2709072000049,
            "range": "± 1.026",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 214.5715884000026,
            "range": "± 5.486",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9892777920080044,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 188.901313599996,
            "range": "± 1.243",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 115.61544560000243,
            "range": "± 0.799",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6338761021044064,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 368.16142360000015,
            "range": "± 1.311",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 478.37818900000286,
            "range": "± 11.094",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7696032805542435,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 680.8312653999991,
            "range": "± 22.395",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 482.7166179999949,
            "range": "± 19.912",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4104160495257825,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 342.1971757999984,
            "range": "± 2.932",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 246.52983239999742,
            "range": "± 4.029",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.388055848935879,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 299.47068339999987,
            "range": "± 2.979",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 269.0670866000005,
            "range": "± 5.019",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.112996342972256,
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
          "id": "ce09b606d6e2cdec50a2451f54dd361a69c7eda6",
          "message": "Merge pull request #334 from patbuc/333-plain-enums\n\n333: Add plain enums",
          "timestamp": "2026-10-02T23:12:45+02:00",
          "tree_id": "33f058629f595ef4af4cf64fcd249dc61821f4f5",
          "url": "https://github.com/patbuc/neon/commit/ce09b606d6e2cdec50a2451f54dd361a69c7eda6"
        },
        "date": 1790975625544,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 196.4914142000012,
            "range": "± 2.395",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 96.58652719999878,
            "range": "± 0.875",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0343563424030395,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 287.47693939999976,
            "range": "± 3.454",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 264.66793020000097,
            "range": "± 3.69",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 1.0861797240895894,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 203.9852035999985,
            "range": "± 2.47",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 167.61696739999792,
            "range": "± 10.948",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2169722836782515,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 109.56608120000055,
            "range": "± 1.269",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 105.95651219999951,
            "range": "± 3.59",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0340665139410001,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 94.98196920000055,
            "range": "± 0.355",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 55.760898799998415,
            "range": "± 1.562",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7033794512652878,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 203.10148939999948,
            "range": "± 2.611",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 241.53279180000027,
            "range": "± 3.44",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8408857773986085,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 285.59241439999994,
            "range": "± 1.521",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 224.52900939999836,
            "range": "± 5.364",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2719622073030978,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 186.24221039999895,
            "range": "± 3.122",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 125.62614540000254,
            "range": "± 2.472",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.482511540945482,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 162.02258859999858,
            "range": "± 2.774",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 126.11390620000122,
            "range": "± 2.081",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.2847321400310174,
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
          "id": "c0eea827a91ebfe17d345fa704866d7373de1b71",
          "message": "Merge pull request #354 from patbuc/335-sleep-native\n\n335: Add a global sleep(ms) native",
          "timestamp": "2026-10-03T07:02:09+02:00",
          "tree_id": "515fa37877a900602b51f26f97f470e8a01a5fe2",
          "url": "https://github.com/patbuc/neon/commit/c0eea827a91ebfe17d345fa704866d7373de1b71"
        },
        "date": 1791003812707,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 370.070237799996,
            "range": "± 1.947",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.65173839999818,
            "range": "± 0.425",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9721119609942264,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 519.4991721999997,
            "range": "± 2.462",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 534.5769864000005,
            "range": "± 8.76",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9717948684967919,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 378.0347841999969,
            "range": "± 0.706",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.46042520000026,
            "range": "± 10.859",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2060048216893589,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 211.36691920000317,
            "range": "± 34.548",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 194.59493500000065,
            "range": "± 6.747",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0861892124787444,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 184.2176726000048,
            "range": "± 8.929",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.5023461999973,
            "range": "± 2.467",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6374562737786638,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 378.5948870000027,
            "range": "± 1.214",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.2672798000025,
            "range": "± 1.555",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8658202991386975,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 592.9657073999977,
            "range": "± 80.848",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 419.73082540000064,
            "range": "± 11.821",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.412728519128671,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 331.0621237999982,
            "range": "± 5.028",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 237.17533060000164,
            "range": "± 8.056",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.3958539573339415,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 289.4904462000028,
            "range": "± 1.905",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.18593160000148,
            "range": "± 9.342",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1904078673274743,
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
          "id": "5f90aa87d868346f4dd849d221d83108e745ff34",
          "message": "Merge pull request #355 from patbuc/337-keep-source-spelling\n\n337: Keep compound assignment and literal spelling in the AST",
          "timestamp": "2026-10-03T07:34:26+02:00",
          "tree_id": "05513fde1eb21a5f6cdb31f9dd7e3e306ad04b30",
          "url": "https://github.com/patbuc/neon/commit/5f90aa87d868346f4dd849d221d83108e745ff34"
        },
        "date": 1791005750251,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 294.53539640000486,
            "range": "± 2.029",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 144.11438599999542,
            "range": "± 1.372",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.043761241157522,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 424.007563400005,
            "range": "± 1.842",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 461.051665399998,
            "range": "± 7.639",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9196530350500862,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 289.6318860000008,
            "range": "± 0.976",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 264.1203879999978,
            "range": "± 5.926",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.096590415428298,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 157.09735699999214,
            "range": "± 0.684",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 164.92124340000487,
            "range": "± 6.387",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9525598628853625,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 142.79392799999755,
            "range": "± 0.731",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.18305879999821,
            "range": "± 0.537",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.803339377942808,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 319.29579559999297,
            "range": "± 1.294",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 398.54746519999935,
            "range": "± 5.518",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8011487300258295,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 534.6686722000072,
            "range": "± 3.863",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 372.2925850000024,
            "range": "± 21.257",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.4361518164537275,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 297.44381079999584,
            "range": "± 6.481",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 217.89606159999266,
            "range": "± 3.213",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.3650719917372103,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 246.99004739999282,
            "range": "± 3.905",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 211.04119500000138,
            "range": "± 6.267",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1703404513037903,
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
          "id": "73133b1d010022ddb5f725c0c6f14c0118d5ce41",
          "message": "Merge pull request #356 from patbuc/338-scanner-trivia\n\n338: Record comments and blank lines in the scanner",
          "timestamp": "2026-10-03T07:41:44+02:00",
          "tree_id": "fe8b9163bbcd0a5601f7d0368f74419e1fc6c1b2",
          "url": "https://github.com/patbuc/neon/commit/73133b1d010022ddb5f725c0c6f14c0118d5ce41"
        },
        "date": 1791006189042,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 366.20931020000853,
            "range": "± 2.078",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 199.58309280000321,
            "range": "± 1.03",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8348714065022378,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 535.7710297999802,
            "range": "± 1.538",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 649.1030875999968,
            "range": "± 21.552",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.82540206638201,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 395.8031414000061,
            "range": "± 0.763",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 356.6766024000003,
            "range": "± 7.882",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1096975207701647,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 209.7767901999987,
            "range": "± 1.118",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 218.5875956000018,
            "range": "± 4.222",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9596921070666508,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 188.56242019998035,
            "range": "± 0.473",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 114.65674260000469,
            "range": "± 2.275",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6445820448414923,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 373.15627279999717,
            "range": "± 3.13",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 479.42101480000474,
            "range": "± 5.139",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7783477596526782,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 666.595086000018,
            "range": "± 3.6",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 492.7420023999957,
            "range": "± 24.51",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3528278140553007,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 365.79710040000464,
            "range": "± 20.519",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 251.30640839997795,
            "range": "± 7.272",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4555820630638432,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 302.7394551999919,
            "range": "± 5.032",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 268.8459385999977,
            "range": "± 5.26",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1260704058855904,
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
          "id": "be8d6133f666aa9fdadc78a651a030a36daac8e0",
          "message": "Merge pull request #357 from patbuc/346-call-depth\n\nRaise the call depth limit",
          "timestamp": "2026-10-03T12:23:51+02:00",
          "tree_id": "168734aaedd67333ca25e271aab5ca729dced7e8",
          "url": "https://github.com/patbuc/neon/commit/be8d6133f666aa9fdadc78a651a030a36daac8e0"
        },
        "date": 1791023112100,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 377.3581874000058,
            "range": "± 11.42",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.24328879999734,
            "range": "± 1.371",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9940373568481924,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 518.4776688,
            "range": "± 1.57",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 525.2435085999991,
            "range": "± 13.367",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9871186607940514,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 377.3695794000048,
            "range": "± 0.852",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 314.1606182000004,
            "range": "± 13.71",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2011995060430025,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 195.1839457999995,
            "range": "± 0.4",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.07934140000202,
            "range": "± 3.082",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0268551246148068,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.70583500000237,
            "range": "± 0.48",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.22624460000168,
            "range": "± 2.277",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6066876629942397,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 391.5120896000019,
            "range": "± 14.458",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.7276463999988,
            "range": "± 1.613",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8843181418272572,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 560.5109603999978,
            "range": "± 10.539",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 407.5139871999994,
            "range": "± 5.123",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3754398082167192,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 331.7489370000004,
            "range": "± 2.578",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 231.0123763999968,
            "range": "± 1.551",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4360656436241266,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 289.80007500000227,
            "range": "± 3.809",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.77830079999967,
            "range": "± 6.335",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.188785359685314,
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
          "id": "9815f97ef27c786a09c62d133ada0e2bf7d7bace",
          "message": "Merge pull request #358 from patbuc/347-sort-comparator\n\nReturn the array from sort() and accept a comparator",
          "timestamp": "2026-10-03T12:41:45+02:00",
          "tree_id": "303891acc696e37019087f35ff56872fec6ed4a3",
          "url": "https://github.com/patbuc/neon/commit/9815f97ef27c786a09c62d133ada0e2bf7d7bace"
        },
        "date": 1791024185195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 369.20143820001385,
            "range": "± 0.612",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.89968900001668,
            "range": "± 4.421",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.964885839699187,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 520.0861103999955,
            "range": "± 0.49",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 539.7743844000047,
            "range": "± 15.839",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9635249938325732,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 379.60008779999725,
            "range": "± 3.263",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 309.0688421999971,
            "range": "± 5.205",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.2282056162567163,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 197.7098329999876,
            "range": "± 4.978",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.71514759999673,
            "range": "± 1.564",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.036676087285219,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.5620056000189,
            "range": "± 1.611",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.22116040002174,
            "range": "± 2.429",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6291064705573854,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 381.83289559999594,
            "range": "± 1.264",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.0325888000025,
            "range": "± 4.696",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8736943316937232,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 556.053429200017,
            "range": "± 2.505",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 404.28345820000686,
            "range": "± 5.893",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3754048500419391,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 317.2950088000107,
            "range": "± 1.41",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 225.65284360000533,
            "range": "± 3.653",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.4061201433935895,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 288.72775000002093,
            "range": "± 2.276",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.64121899999054,
            "range": "± 10.212",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.1659115197621117,
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
          "id": "ea4cbf63945540af9748822b3a2f32235acd50cb",
          "message": "Merge pull request #359 from patbuc/344-integers\n\nRepresent integers beyond 2^53 exactly",
          "timestamp": "2026-10-03T15:30:38+02:00",
          "tree_id": "2ff79ba8a21a8f84607c3e0034081e56e19f357c",
          "url": "https://github.com/patbuc/neon/commit/ea4cbf63945540af9748822b3a2f32235acd50cb"
        },
        "date": 1791034305608,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 260.67581580001047,
            "range": "± 0.976",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 155.59813100000497,
            "range": "± 4.501",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6753145691705136,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 340.5327707999959,
            "range": "± 0.291",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 493.71559099999445,
            "range": "± 13.948",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6897346914045493,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 269.37362379999286,
            "range": "± 0.285",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 281.5679350000039,
            "range": "± 6.169",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9566914066404227,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 148.61431820000348,
            "range": "± 1.284",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 167.587685400008,
            "range": "± 2.459",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8867854332213145,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.64329320000388,
            "range": "± 0.531",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.06290139999646,
            "range": "± 2.38",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.585722148373475,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 283.8262527999859,
            "range": "± 5.329",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 365.856952199988,
            "range": "± 1.847",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7757847735112555,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 442.32423800000333,
            "range": "± 2.345",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 374.86584219999486,
            "range": "± 9.908",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1799534345517102,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 200.72435880000512,
            "range": "± 1.838",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 183.91118300000926,
            "range": "± 3.612",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.091420084008676,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 191.44895800001223,
            "range": "± 1.188",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 203.53889100000515,
            "range": "± 18.83",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9406013615354099,
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
          "id": "312c1ba6d0f1ba1ebb657036450c4298c1be04b6",
          "message": "Merge pull request #360 from patbuc/351-math-functions\n\nAdd Math.round, sign, gcd, lcm and mod",
          "timestamp": "2026-10-03T17:22:37+02:00",
          "tree_id": "a0bc194c5bfa647c4d2721139be72ccb2567770d",
          "url": "https://github.com/patbuc/neon/commit/312c1ba6d0f1ba1ebb657036450c4298c1be04b6"
        },
        "date": 1791041026387,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 261.61944119999987,
            "range": "± 1.477",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 154.32774580000057,
            "range": "± 3.713",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.695219740584061,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 340.52540940000426,
            "range": "± 0.222",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 489.6169130000004,
            "range": "± 27.887",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6954935590633977,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 268.46004939999375,
            "range": "± 0.165",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 284.680025800003,
            "range": "± 9.058",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9430238340240831,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 149.27346700000328,
            "range": "± 1.842",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 167.97018239999773,
            "range": "± 5.32",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8886902714943131,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.8851442000023,
            "range": "± 0.278",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.74422879999975,
            "range": "± 1.688",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5942375483047462,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 286.495372600001,
            "range": "± 10.726",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 373.1016121999943,
            "range": "± 32.051",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7678749253070245,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 443.40052920000517,
            "range": "± 1.309",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 369.47089599999856,
            "range": "± 26.4",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2000959588438243,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 204.66800619999788,
            "range": "± 1.664",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 189.65097380000202,
            "range": "± 7.991",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.079182469243908,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 189.59129359999736,
            "range": "± 1.311",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 208.24614940000004,
            "range": "± 4.052",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.910419204130539,
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
          "id": "bc92c61908b75185e4d16860f08197dc12547568",
          "message": "Merge pull request #361 from patbuc/348-split-charcodes\n\nSplit on whitespace and convert char codes",
          "timestamp": "2026-10-03T17:44:51+02:00",
          "tree_id": "8409395b7c23ae9f579129e1ab7afefc506a5db6",
          "url": "https://github.com/patbuc/neon/commit/bc92c61908b75185e4d16860f08197dc12547568"
        },
        "date": 1791042367458,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 357.2356319999926,
            "range": "± 1.175",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.16442600004848,
            "range": "± 1.501",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8985290662747303,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 439.4062477999796,
            "range": "± 1.942",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 529.2242135999913,
            "range": "± 4.69",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8302837181446506,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 350.8351149999271,
            "range": "± 1.339",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.9842191999833,
            "range": "± 13.366",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1173654392371635,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 190.7841835999534,
            "range": "± 0.47",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 191.77851240001473,
            "range": "± 3.193",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.994815223104936,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.40772900001502,
            "range": "± 1.139",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.47383919997264,
            "range": "± 0.759",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6205490763500998,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 385.6961885999681,
            "range": "± 10.075",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.8858256000058,
            "range": "± 3.257",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8808145092878361,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 535.8308751999175,
            "range": "± 0.78",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.1880517999889,
            "range": "± 12.593",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2968208370635461,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 253.07716100005564,
            "range": "± 1.81",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 224.14456400006202,
            "range": "± 6.646",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1290800744111984,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.82894539996414,
            "range": "± 2.772",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.88518520002253,
            "range": "± 3.689",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0329773713287738,
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
          "id": "0aaaf2e07c9d2ebdfbdacf04d9f9c0a0fdfd4672",
          "message": "Merge pull request #362 from patbuc/349-string-helpers\n\nAdd repeat, padStart, padEnd, lastIndexOf, includes to String",
          "timestamp": "2026-10-03T18:03:37+02:00",
          "tree_id": "a823d99fea1b611b01f6b5ac7f4ef21e9f920eec",
          "url": "https://github.com/patbuc/neon/commit/0aaaf2e07c9d2ebdfbdacf04d9f9c0a0fdfd4672"
        },
        "date": 1791043492126,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 276.45683139999164,
            "range": "± 0.992",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 154.12355959999786,
            "range": "± 30.241",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.793735053339603,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 352.0110625999905,
            "range": "± 0.26",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 454.55363319999833,
            "range": "± 8.977",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7744104037226114,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 278.5698463999893,
            "range": "± 0.429",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 254.27890319998596,
            "range": "± 2.928",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0955287398770117,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 150.7222679999927,
            "range": "± 0.205",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 157.70119719998092,
            "range": "± 3.932",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9557458705203218,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 138.39763120000725,
            "range": "± 0.218",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 76.56261219998441,
            "range": "± 1.056",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.8076398809187368,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 312.2551478000105,
            "range": "± 1.234",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 384.475419599994,
            "range": "± 1.152",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8121589362588614,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 430.3141787999948,
            "range": "± 1.156",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 348.9992003999987,
            "range": "± 7.571",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2329947412681705,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 197.3958717999949,
            "range": "± 0.303",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 195.6941130000132,
            "range": "± 3.252",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0086960142739785,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 198.0818657999862,
            "range": "± 2.256",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 205.71749219999447,
            "range": "± 4.967",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9628829502131735,
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
          "id": "b3c14624aa20a94ad96d8b654347373a9b1bf5f0",
          "message": "Merge pull request #363 from patbuc/350-array-helpers\n\nAdd find, some, every, flat and copy to Array, plus Array(n, init)",
          "timestamp": "2026-10-03T18:33:36+02:00",
          "tree_id": "551d77f1c6ee9b861753fb2ae6df9b5a7ce0452b",
          "url": "https://github.com/patbuc/neon/commit/b3c14624aa20a94ad96d8b654347373a9b1bf5f0"
        },
        "date": 1791045281921,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 244.80862280000224,
            "range": "± 6.138",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 121.29740059999676,
            "range": "± 0.309",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0182511874867726,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 304.8954417999937,
            "range": "± 1.873",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 397.259641800008,
            "range": "± 6.853",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7674966438032658,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 242.66895619999787,
            "range": "± 2.548",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 220.98699800000077,
            "range": "± 2.255",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0981141804550738,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 131.37643659999867,
            "range": "± 0.947",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 136.25267799999392,
            "range": "± 2.918",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9642117757128013,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 120.5947005999974,
            "range": "± 1.599",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 67.22236579999503,
            "range": "± 3.358",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.793966921051242,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 271.4429234000022,
            "range": "± 2.633",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 338.654593800004,
            "range": "± 7.986",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8015332683197135,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 375.57400400000347,
            "range": "± 1.085",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 297.5597283999946,
            "range": "± 4.727",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2621802218314242,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 172.64834440000527,
            "range": "± 1.921",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 170.25288219999766,
            "range": "± 3.161",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0140700243605487,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 174.61953559999301,
            "range": "± 3.239",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 181.44774239999606,
            "range": "± 2.892",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9623681909199483,
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
          "id": "42743504afe6480e860a63f5d3e172ac0a7e7762",
          "message": "Merge pull request #364 from patbuc/352-stdin\n\nRead standard input",
          "timestamp": "2026-10-03T18:51:26+02:00",
          "tree_id": "8beccf550856fd21846afc50fa6afb1d0949873f",
          "url": "https://github.com/patbuc/neon/commit/42743504afe6480e860a63f5d3e172ac0a7e7762"
        },
        "date": 1791046367492,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 361.1962910000017,
            "range": "± 4.017",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.08516959999793,
            "range": "± 0.982",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.920386874564116,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 441.06475840000314,
            "range": "± 0.919",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 545.4305402000045,
            "range": "± 11.424",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8086543123131107,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 355.4301836000036,
            "range": "± 1.781",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 307.9258695999954,
            "range": "± 3.476",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1542719163599917,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 188.80928019999885,
            "range": "± 0.366",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 210.14405059999604,
            "range": "± 42.097",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8984754965030755,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 183.6275814000004,
            "range": "± 14.378",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.64679600000227,
            "range": "± 2.17",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6447187736582844,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 389.65948759999947,
            "range": "± 6.544",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 439.67261539999924,
            "range": "± 2.848",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8862491634724634,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 538.1982983999961,
            "range": "± 2.662",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 410.3794092000044,
            "range": "± 6.854",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.311465162078094,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 253.39700739999955,
            "range": "± 2.34",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 223.4958388000024,
            "range": "± 5.41",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1337884801817475,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 256.4138383999989,
            "range": "± 6.224",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 242.26170400000058,
            "range": "± 5.904",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0584167211174171,
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
          "id": "551dab8c115e193153f8c6326cddea70f16ff766",
          "message": "Merge pull request #365 from patbuc/353-priority-queue\n\nAdd a priority queue",
          "timestamp": "2026-10-03T19:16:52+02:00",
          "tree_id": "d278febcab9e17fc7570fdded87bc226eb27188e",
          "url": "https://github.com/patbuc/neon/commit/551dab8c115e193153f8c6326cddea70f16ff766"
        },
        "date": 1791047890579,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 285.64576639999757,
            "range": "± 0.558",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 147.1876724000026,
            "range": "± 1.724",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.940690831931333,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 364.0803370000043,
            "range": "± 0.767",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 471.2466769999992,
            "range": "± 6.313",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7725897173806595,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 286.4571717999979,
            "range": "± 0.458",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 262.97663340000383,
            "range": "± 1.884",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0892875465641798,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 155.7546464000012,
            "range": "± 0.554",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 162.8638058000007,
            "range": "± 2.793",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9563490527248906,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 143.77222640000014,
            "range": "± 0.497",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.43699999999865,
            "range": "± 0.303",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.8098899303851175,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 321.9280267999977,
            "range": "± 0.329",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 400.7052140000013,
            "range": "± 0.715",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8034036382666003,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 454.2196529999984,
            "range": "± 3.09",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 361.5718979999997,
            "range": "± 14.971",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2562360501810867,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 220.01255859999844,
            "range": "± 0.727",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 209.05908359999898,
            "range": "± 3.339",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0523941596384168,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 208.7969552000004,
            "range": "± 0.375",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 213.16497620000234,
            "range": "± 5.77",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9795087303840025,
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
          "id": "6128ebe4b426ba76dff1967c16620c42bf49d73a",
          "message": "Merge pull request #366 from patbuc/345-array-map-keys\n\nAccept arrays as map keys and set elements",
          "timestamp": "2026-10-03T19:56:39+02:00",
          "tree_id": "afae49ffe2917d602aa9cf55a09fc3986a584df6",
          "url": "https://github.com/patbuc/neon/commit/6128ebe4b426ba76dff1967c16620c42bf49d73a"
        },
        "date": 1791050267366,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 261.14781899999997,
            "range": "± 1.009",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 153.2771996000008,
            "range": "± 0.62",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7037616793724262,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 342.0925464000007,
            "range": "± 0.98",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 487.7918264000016,
            "range": "± 15.987",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7013084842456466,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 271.80168359999755,
            "range": "± 0.485",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 275.4961347999995,
            "range": "± 4.027",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9865898256515142,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 148.121860400002,
            "range": "± 0.659",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 173.73232680000115,
            "range": "± 9.259",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8525866378945027,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.2879070000049,
            "range": "± 0.507",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.34182279999538,
            "range": "± 0.43",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5766926987158814,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 285.81860600000084,
            "range": "± 2.001",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 368.1050733999996,
            "range": "± 1.947",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7764592956028548,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 467.4869176000044,
            "range": "± 12.215",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 371.6470541999996,
            "range": "± 15.065",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2578787113120211,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 219.0519261999981,
            "range": "± 2.822",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 187.75546600000155,
            "range": "± 2.553",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1666873453367066,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 192.32441940000058,
            "range": "± 3.837",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 215.0388840000005,
            "range": "± 6.281",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8943704311635106,
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
          "id": "12c40d43632721432fbad164319ca7d213aa412a",
          "message": "Merge pull request #368 from patbuc/339-formatter-core\n\n339: Add the source formatter core",
          "timestamp": "2026-10-04T13:41:53+02:00",
          "tree_id": "cd330f8eefe4031cf5bc3b2b9c93499f9835af9d",
          "url": "https://github.com/patbuc/neon/commit/12c40d43632721432fbad164319ca7d213aa412a"
        },
        "date": 1791114193963,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 379.86836859999755,
            "range": "± 30.002",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.35363320000147,
            "range": "± 2.292",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0275473825185184,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 445.0175253999987,
            "range": "± 6.415",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.3255220000002,
            "range": "± 5.429",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8375609809310055,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.8303753999978,
            "range": "± 1.17",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.1664661999962,
            "range": "± 6.01",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.137551649998462,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.5891472000045,
            "range": "± 0.807",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.5777169999983,
            "range": "± 2.251",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0053634661406268,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.96933739999758,
            "range": "± 0.843",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.97480340000538,
            "range": "± 3.317",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5893695009603297,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 391.00528840000095,
            "range": "± 6.603",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 439.97168680000414,
            "range": "± 3.318",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8887055693148236,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 535.017811199998,
            "range": "± 2.004",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 408.2851268000013,
            "range": "± 8.196",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3104024028337353,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 272.8711785999991,
            "range": "± 9.426",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 225.39530259999765,
            "range": "± 8.367",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2106338306626359,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.22291300000074,
            "range": "± 2.287",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.30990680000184,
            "range": "± 10.963",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0201923804279673,
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
          "id": "8b651ea67d2c29ef45ef900d9d5eba2cfd157b40",
          "message": "Merge pull request #371 from patbuc/340-neon-fmt\n\n340: Add the neon fmt command",
          "timestamp": "2026-10-04T14:07:58+02:00",
          "tree_id": "eefb57062b7eeac9bd664a3245f7860cf1ab98a0",
          "url": "https://github.com/patbuc/neon/commit/8b651ea67d2c29ef45ef900d9d5eba2cfd157b40"
        },
        "date": 1791115751195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 276.02737819999845,
            "range": "± 4.447",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 179.17306699999926,
            "range": "± 2.449",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.5405628916314726,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 312.7341813999976,
            "range": "± 6.39",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 486.83671260000096,
            "range": "± 6.401",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6423800286749307,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 262.4286679999983,
            "range": "± 5.607",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 299.9090683999981,
            "range": "± 4.612",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8750274521542276,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 144.85622859999978,
            "range": "± 0.876",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.23779420000335,
            "range": "± 3.771",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.769538493667693,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 128.69146939999894,
            "range": "± 0.879",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 90.87137200000086,
            "range": "± 2.066",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.4161937535178595,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 329.1642216000042,
            "range": "± 2.286",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 430.18603860000155,
            "range": "± 5.589",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7651671418050596,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 447.0507996000009,
            "range": "± 7.968",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 414.28918819999865,
            "range": "± 6.868",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0790790885524788,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 214.25841500000047,
            "range": "± 3.552",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 236.96062619999907,
            "range": "± 3.994",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.904194162700948,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 194.8143783999967,
            "range": "± 2.849",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 231.9952139999998,
            "range": "± 6.678",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8397344714188667,
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
          "id": "94d019b11fd9f8497b4f919e9546c2e9154ebeae",
          "message": "Merge pull request #372 from patbuc/341-format-all-n\n\n341: Format all .n files and enforce neon fmt --check",
          "timestamp": "2026-10-04T14:20:40+02:00",
          "tree_id": "48ee45e12316e750fbfeb765ebefaa06e70f14b7",
          "url": "https://github.com/patbuc/neon/commit/94d019b11fd9f8497b4f919e9546c2e9154ebeae"
        },
        "date": 1791116514808,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 260.3559688000004,
            "range": "± 0.758",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 151.86499859999998,
            "range": "± 0.347",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.714390881375878,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 342.081506400001,
            "range": "± 0.415",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 493.51378599999975,
            "range": "± 7.983",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6931549150280498,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 271.99630080000077,
            "range": "± 0.512",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 282.33261960000107,
            "range": "± 11.32",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9633895693149293,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 148.980534399999,
            "range": "± 0.327",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 171.09348339999997,
            "range": "± 2.034",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8707551651847367,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.27603119999787,
            "range": "± 0.309",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.13885879999646,
            "range": "± 0.8",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.580188728288861,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 286.1457169999994,
            "range": "± 6.173",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 365.19052360000046,
            "range": "± 2.554",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7835518681569619,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 447.53033360000245,
            "range": "± 4.567",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 376.96790040000394,
            "range": "± 2.899",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1871841955909883,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 207.2120438000013,
            "range": "± 2.222",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 183.07337319999988,
            "range": "± 4.063",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1318524380584334,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 191.28427839999915,
            "range": "± 2.212",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 215.80054720000135,
            "range": "± 4.808",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8863938524804535,
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
          "id": "2c0cd08e7f489385c8dc7c100e779cb2c6ab41f1",
          "message": "Merge pull request #373 from patbuc/342-wasm-format\n\n342: Export the formatter to wasm",
          "timestamp": "2026-10-04T14:32:48+02:00",
          "tree_id": "f490c9d5e47ffcffca15004e19c91752cde2e684",
          "url": "https://github.com/patbuc/neon/commit/2c0cd08e7f489385c8dc7c100e779cb2c6ab41f1"
        },
        "date": 1791117241122,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 251.87946359999955,
            "range": "± 2.042",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 126.60625299999992,
            "range": "± 0.591",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9894709592266324,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 314.2250843999932,
            "range": "± 1.563",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 408.62030680000316,
            "range": "± 9.023",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7689903785270976,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 242.62553159999243,
            "range": "± 0.527",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 233.2403090000014,
            "range": "± 12.145",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0402384246540806,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 133.22485920000418,
            "range": "± 0.485",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 139.61586780000061,
            "range": "± 1.661",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9542243392480894,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 122.1886758000096,
            "range": "± 0.326",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 70.08599480000157,
            "range": "± 4.316",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7434107363202735,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 280.42623000001186,
            "range": "± 0.751",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 347.4652644000116,
            "range": "± 4.279",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8070626296537585,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 391.0909205999985,
            "range": "± 3.379",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 317.49256260000607,
            "range": "± 15.385",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2318112821203802,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 190.07616420000204,
            "range": "± 0.947",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 180.76878759999317,
            "range": "± 5.04",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.051487741460126,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 178.44602299999792,
            "range": "± 0.867",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 185.24231200000258,
            "range": "± 4.959",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9633113572885844,
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
          "id": "4998d9291dd3b70b2435fa7d09a9a57676ac3479",
          "message": "Merge pull request #374 from patbuc/102-repl-state\n\nKeep REPL state between lines",
          "timestamp": "2026-10-04T14:39:02+02:00",
          "tree_id": "865c9cad540fd574ebb0e299d6365ae9fd1e1601",
          "url": "https://github.com/patbuc/neon/commit/4998d9291dd3b70b2435fa7d09a9a57676ac3479"
        },
        "date": 1791117601977,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 199.20630839999944,
            "range": "± 2.688",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 105.9360448000021,
            "range": "± 2.339",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.880439361088885,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 250.69681860000088,
            "range": "± 12.633",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 276.66086780000256,
            "range": "± 4.664",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9061520720061825,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 199.97737879999846,
            "range": "± 3.593",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 172.11724199999878,
            "range": "± 2.219",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1618672044489295,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 105.8334682000023,
            "range": "± 0.322",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 108.39534160000142,
            "range": "± 5.059",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9763654658753428,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 99.71213339999991,
            "range": "± 1.402",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 57.651666400001034,
            "range": "± 0.553",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.729562033960533,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 211.25865559999966,
            "range": "± 1.947",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 254.26623399999642,
            "range": "± 4.115",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8308561159560126,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 316.3613477999917,
            "range": "± 22.44",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 241.53518419999784,
            "range": "± 11.776",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.30979405277053,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 158.69887119999646,
            "range": "± 6.103",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 134.39702080000018,
            "range": "± 3.368",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1808213474922225,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 139.90499419999765,
            "range": "± 1.62",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 132.10564379999994,
            "range": "± 2.26",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.059038737298805,
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
          "id": "1599e9767eb9aa72629f2dc446480feaff296502",
          "message": "Merge pull request #414 from patbuc/408-native-context-trait\n\n408: Break the common → vm dependency behind NativeContext",
          "timestamp": "2026-10-04T22:15:33+02:00",
          "tree_id": "857529f7918b1643fbeed894417ed986a487974d",
          "url": "https://github.com/patbuc/neon/commit/1599e9767eb9aa72629f2dc446480feaff296502"
        },
        "date": 1791145017957,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 366.19261080000456,
            "range": "± 0.938",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.1817527999999,
            "range": "± 2.763",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9459517479848063,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 442.8762173999928,
            "range": "± 0.931",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 534.9839610000004,
            "range": "± 6.027",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8278308317358929,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 365.4144398000028,
            "range": "± 23.059",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.6809263999935,
            "range": "± 6.801",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1761727507196285,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.9212796000029,
            "range": "± 1.699",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 191.0306303999903,
            "range": "± 5.138",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9941928119188814,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.29180699999847,
            "range": "± 3.005",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.86578419999569,
            "range": "± 1.095",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6171969403704036,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 388.71616459999814,
            "range": "± 2.592",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 443.91366840000046,
            "range": "± 9.964",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8756571204510307,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 543.7055612000051,
            "range": "± 21.16",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 424.07375960001445,
            "range": "± 18.318",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2821014007394071,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 269.17583100000115,
            "range": "± 2.797",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 221.8867211999907,
            "range": "± 3.144",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2131227571630476,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 246.61602359999506,
            "range": "± 1.446",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 246.0668960000021,
            "range": "± 8.452",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0022316191609657,
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
          "id": "1815c038daddf778ed84c7f4fc3031a984fddf33",
          "message": "Merge pull request #415 from patbuc/409-layer-boundary-test\n\n409: Add a layer-boundary test over src/",
          "timestamp": "2026-10-04T22:25:56+02:00",
          "tree_id": "298a44dfc8e6ceb6749c6bae58fad31a6ccb5a76",
          "url": "https://github.com/patbuc/neon/commit/1815c038daddf778ed84c7f4fc3031a984fddf33"
        },
        "date": 1791145639425,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 369.0255608000001,
            "range": "± 3.15",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.79555139999877,
            "range": "± 1.075",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9755586149360649,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 442.3792693999985,
            "range": "± 0.2",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 539.8500860000013,
            "range": "± 5.73",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8194483632998764,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.2286090000023,
            "range": "± 1.113",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 307.86994560000664,
            "range": "± 3.673",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1440824738950897,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.47790599999905,
            "range": "± 0.63",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 200.00274259999742,
            "range": "± 16.031",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9473765386255333,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.39721680000287,
            "range": "± 0.168",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.70631300000002,
            "range": "± 1.9",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6024128343972834,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 387.3113116000013,
            "range": "± 1.722",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 444.8439102000009,
            "range": "± 10.958",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8706678965794248,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 536.2054776000036,
            "range": "± 3.063",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 426.29391120000264,
            "range": "± 16.559",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.257830486226284,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 263.57186160000197,
            "range": "± 2.652",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 223.9743209999972,
            "range": "± 5.981",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.176795002316383,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 249.09867320000103,
            "range": "± 5.944",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.16039260000355,
            "range": "± 3.131",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0372179629756209,
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
          "id": "ef2006c1f5df9965062ee6c08db808b6a50d0c7a",
          "message": "Merge pull request #416 from patbuc/410-arch-edit-hook\n\n410: Run the layer-boundary test from the edit hook",
          "timestamp": "2026-10-04T22:32:40+02:00",
          "tree_id": "0f88d81dda98bea6822a9b1ae4a17ef786636004",
          "url": "https://github.com/patbuc/neon/commit/ef2006c1f5df9965062ee6c08db808b6a50d0c7a"
        },
        "date": 1791146037482,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 366.7684449999996,
            "range": "± 1.195",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 191.70162640000115,
            "range": "± 6.841",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9132255259781008,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 443.112061599993,
            "range": "± 1.315",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 522.9565433999966,
            "range": "± 6.535",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8473210005540891,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 356.59684379999703,
            "range": "± 5.846",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 305.58162779999805,
            "range": "± 4.983",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1669446437839786,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.0838255999995,
            "range": "± 0.237",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 199.41211439999904,
            "range": "± 8.684",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9482063121838119,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.6595754000009,
            "range": "± 3.31",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.67393920000086,
            "range": "± 0.272",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.62332322043164,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 387.05852820000075,
            "range": "± 1.404",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 434.677577399998,
            "range": "± 1.733",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8904497225625758,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 538.882387000001,
            "range": "± 5.223",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.8766992000036,
            "range": "± 5.163",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3244365874466282,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 264.51930859999777,
            "range": "± 3.198",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 218.31789820000154,
            "range": "± 2.067",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2116244741311637,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 249.2366279999942,
            "range": "± 4.255",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 238.58686320000402,
            "range": "± 5.394",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.044636844867115,
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
          "id": "99b0b6ad941b961f10d6123e5818d44fad17631c",
          "message": "Merge pull request #417 from patbuc/411-deny-print-lints\n\n411: Deny print_stdout and print_stderr outside tests",
          "timestamp": "2026-10-04T22:42:09+02:00",
          "tree_id": "f2805eb3310f103d22c0baadc72d949a811725e9",
          "url": "https://github.com/patbuc/neon/commit/99b0b6ad941b961f10d6123e5818d44fad17631c"
        },
        "date": 1791146616326,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 367.72888120000573,
            "range": "± 3.508",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 196.10351039999614,
            "range": "± 7.009",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8751774532232595,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 449.18675620000386,
            "range": "± 16.488",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 534.4476391999962,
            "range": "± 11.437",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8404691559165317,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.1499607999999,
            "range": "± 2.427",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.23507120000045,
            "range": "± 7.318",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1242354167140893,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.48666799999785,
            "range": "± 0.407",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.94310359999906,
            "range": "± 2.785",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0028768681663531,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.833065599998,
            "range": "± 0.165",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.62465699999677,
            "range": "± 0.785",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6075355207655306,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 386.93698079999876,
            "range": "± 1.267",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.30486559999633,
            "range": "± 3.601",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.874819634360364,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 535.7536750000065,
            "range": "± 3.016",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 417.1713500000038,
            "range": "± 8.739",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2842532810558578,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 268.0855627999989,
            "range": "± 4.569",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 228.26759339999683,
            "range": "± 13.982",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1744354895363025,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.64542379999932,
            "range": "± 0.609",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.5533687999963,
            "range": "± 6.45",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0336393335099414,
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
          "id": "22702b84710e24c1ed7fab7bc3d081e9aed51de0",
          "message": "Merge pull request #418 from patbuc/412-drop-tracing\n\n412: Remove the unused tracing dependencies",
          "timestamp": "2026-10-04T22:47:32+02:00",
          "tree_id": "0009608939c75082929dbfc85b26e923af2fb7a4",
          "url": "https://github.com/patbuc/neon/commit/22702b84710e24c1ed7fab7bc3d081e9aed51de0"
        },
        "date": 1791146930189,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 366.5551978000053,
            "range": "± 1.205",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.54347420000192,
            "range": "± 1.786",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.964985370686323,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 442.4326344000008,
            "range": "± 1.207",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 534.2361326000002,
            "range": "± 7.577",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8281593239431154,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 358.3816196000015,
            "range": "± 7.868",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.35657279999975,
            "range": "± 10.368",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1510327737009372,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.1747247999973,
            "range": "± 0.378",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.98729360000277,
            "range": "± 7.035",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9905094796316574,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.75436279999894,
            "range": "± 0.847",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.97872959999881,
            "range": "± 1.421",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.61626128476393,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 388.3063129999982,
            "range": "± 0.868",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 433.6263889999941,
            "range": "± 3.094",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8954858902740892,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 534.3105436000002,
            "range": "± 1.275",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 417.56711399999915,
            "range": "± 22.053",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2795800380007925,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 263.7437697999985,
            "range": "± 3.879",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 218.6774287999981,
            "range": "± 2.195",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2060859287001129,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 247.79817320000177,
            "range": "± 1.95",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.4145629999989,
            "range": "± 3.954",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0180088247226313,
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
          "id": "14c07220994aac693c0af60fa2f81d0b92d152f7",
          "message": "Merge pull request #419 from patbuc/413-deny-unwrap-expect\n\n413: Deny unwrap_used and expect_used outside tests",
          "timestamp": "2026-10-04T23:31:25+02:00",
          "tree_id": "7c0c5338232c0cd6ba7eab1bdf42e9afc44eef25",
          "url": "https://github.com/patbuc/neon/commit/14c07220994aac693c0af60fa2f81d0b92d152f7"
        },
        "date": 1791149567198,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 375.99522060000083,
            "range": "± 17.072",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.58312740000258,
            "range": "± 2.457",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9937903553931393,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 443.7649205999975,
            "range": "± 1.044",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 522.7416696000034,
            "range": "± 6.104",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.848918206462404,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 354.37084540000114,
            "range": "± 0.332",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 322.8870750000027,
            "range": "± 19.423",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.097507063111765,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.39390580000293,
            "range": "± 0.351",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.68926339999734,
            "range": "± 4.166",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9932069715048548,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.85188560000438,
            "range": "± 2.043",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.09611660000189,
            "range": "± 1.5",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6335897319016,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.8875961999978,
            "range": "± 2.623",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.0691878000031,
            "range": "± 2.904",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8989139641199733,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 532.4966783999997,
            "range": "± 1.026",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 433.0529102000014,
            "range": "± 24.146",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2296342221879335,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 263.8024087999952,
            "range": "± 6.807",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 217.15542200000186,
            "range": "± 6.143",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2148092199143568,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.23115460000338,
            "range": "± 1.915",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.66699100000346,
            "range": "± 11.902",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0187311526328153,
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
          "id": "9ab7289486df791a782fa13b26f7f553b09eb538",
          "message": "Merge pull request #420 from patbuc/393-leading-dot-chains\n\n393: Continue a statement on a line that starts with .",
          "timestamp": "2026-10-04T23:55:56+02:00",
          "tree_id": "d619a531f551fa6fd91a0102fbf259c74b21456f",
          "url": "https://github.com/patbuc/neon/commit/9ab7289486df791a782fa13b26f7f553b09eb538"
        },
        "date": 1791151038739,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 369.0163236000018,
            "range": "± 1.885",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.88538459999893,
            "range": "± 2.099",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9536520752066906,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 446.73064399999873,
            "range": "± 2.931",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 532.5568940000011,
            "range": "± 9.737",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.838841162386676,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 353.51179300000126,
            "range": "± 0.401",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 326.37324260000184,
            "range": "± 28.812",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0831518851968511,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.30356480000228,
            "range": "± 1.055",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 189.98624140000686,
            "range": "± 2.275",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9964067050594088,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.63731320000034,
            "range": "± 0.482",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.65262559999951,
            "range": "± 0.77",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6053601280293626,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 394.0879958000039,
            "range": "± 4.017",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.5802073999961,
            "range": "± 5.92",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8904329412178937,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 541.873249599999,
            "range": "± 4.5",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 441.7516320000004,
            "range": "± 41.884",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2266468539045452,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 269.0416177999964,
            "range": "± 3.478",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 233.86941720000038,
            "range": "± 3.456",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1503924755151609,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 249.31975400000113,
            "range": "± 2.516",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 242.70217279999713,
            "range": "± 5.099",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.027266262694143,
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
          "id": "d23bb020011aa077fad8d227e434f67c6f458f6c",
          "message": "Merge pull request #430 from patbuc/394-paren-free-conditions\n\n394: Drop parentheses from if/while/for and require braces",
          "timestamp": "2026-10-05T01:06:35+02:00",
          "tree_id": "de61b373608ac40ff8589c50fa68f615c9fd7aea",
          "url": "https://github.com/patbuc/neon/commit/d23bb020011aa077fad8d227e434f67c6f458f6c"
        },
        "date": 1791155270552,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 268.353137400004,
            "range": "± 12.152",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 152.8144937999997,
            "range": "± 0.436",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7560712385777935,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 342.9974208000033,
            "range": "± 2.741",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 503.2905994000032,
            "range": "± 8.388",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6815096908404547,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 273.6042934000011,
            "range": "± 0.388",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 283.85693080000465,
            "range": "± 15.954",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9638809685882668,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 148.58193260000405,
            "range": "± 0.34",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 167.27942260000077,
            "range": "± 12.233",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8882260010861812,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.49386500000003,
            "range": "± 0.913",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.31344940000463,
            "range": "± 0.452",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5795313844913945,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 291.573474599997,
            "range": "± 15.859",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 378.1655164,
            "range": "± 5.26",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7710207883988784,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 494.3883987999982,
            "range": "± 10.21",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 380.01751380000144,
            "range": "± 29.613",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3009621421295565,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 213.02822640000443,
            "range": "± 3.143",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 196.01816019999774,
            "range": "± 2.689",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0867780117038712,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 198.7656945999987,
            "range": "± 12.309",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 217.44170300000008,
            "range": "± 10.018",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9141102735016688,
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
          "id": "c3d431a3284035e90dfc5aa846fde35c9e55b896",
          "message": "Merge pull request #431 from patbuc/395-implicit-return\n\n395: Return a function's last expression and add fn f(x) = expr",
          "timestamp": "2026-10-05T01:49:30+02:00",
          "tree_id": "a3b066c2d9b413fee2186e8fa542a78b20256e5c",
          "url": "https://github.com/patbuc/neon/commit/c3d431a3284035e90dfc5aa846fde35c9e55b896"
        },
        "date": 1791157846369,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 365.5532285999982,
            "range": "± 1.2",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.51521219999745,
            "range": "± 1.867",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9494590562077243,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 450.84793240000636,
            "range": "± 1.314",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 532.8744348000015,
            "range": "± 4.722",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8460678594371274,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 353.41841940000336,
            "range": "± 0.848",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 309.48043599999835,
            "range": "± 5.132",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1419733795386189,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 186.9763669999969,
            "range": "± 0.368",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 196.865558799999,
            "range": "± 8.148",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9497667755584979,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.53235600000323,
            "range": "± 1.288",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 113.90048019999597,
            "range": "± 4.106",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.558662050311615,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 390.03425180000306,
            "range": "± 3.181",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.4801724000008,
            "range": "± 8.348",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8915472663830429,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 553.0395983999995,
            "range": "± 49.576",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 408.7891712000044,
            "range": "± 9.831",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3528724275561084,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 257.8594112000019,
            "range": "± 1.399",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 223.07448180000335,
            "range": "± 3.783",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1559341486274743,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 250.82498200000316,
            "range": "± 0.992",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 242.08829060000596,
            "range": "± 7.2",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.036088864018758,
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
          "id": "d878273d7fe08dfb1a49575356405adb380184dc",
          "message": "Merge pull request #432 from patbuc/396-compound-field-index\n\n396: Allow compound assignment on fields and indexes, remove ++/--",
          "timestamp": "2026-10-05T02:56:38+02:00",
          "tree_id": "ecdd426f3d1526fbbdd558ee826e31805b4c1081",
          "url": "https://github.com/patbuc/neon/commit/d878273d7fe08dfb1a49575356405adb380184dc"
        },
        "date": 1791161879318,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 364.636154599998,
            "range": "± 1.489",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.91108860000259,
            "range": "± 3.856",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9301998485227776,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 441.74081560000786,
            "range": "± 1.56",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 528.8271192000025,
            "range": "± 9.185",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.835321789601606,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 351.9726960000014,
            "range": "± 0.519",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.4724369999958,
            "range": "± 9.238",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1228186419465203,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 192.7701634000016,
            "range": "± 0.283",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.37638220000122,
            "range": "± 5.893",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0020469311019218,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 183.47325539999702,
            "range": "± 7.253",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.48645179999426,
            "range": "± 0.379",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6605950540626064,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 391.22037199999795,
            "range": "± 1.366",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 448.50474699999836,
            "range": "± 3.138",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8722769928675903,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 537.0452933999957,
            "range": "± 10.296",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.66575399999874,
            "range": "± 15.541",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2982590127583955,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 282.24779820000094,
            "range": "± 3.749",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 239.2420935999951,
            "range": "± 4.695",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1797581017323397,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 251.8427289999977,
            "range": "± 4.973",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 250.1942692,
            "range": "± 8.537",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0065887192591128,
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
          "id": "aa79ee0aac4a1a4d028577d8e0f5b298d25d8395",
          "message": "Merge pull request #433 from patbuc/397-null-safe-ops\n\n397: Add ?. and ??",
          "timestamp": "2026-10-05T03:54:20+02:00",
          "tree_id": "adb5cd771e7176d7ae01e6ca09c3e7626d2d2add",
          "url": "https://github.com/patbuc/neon/commit/aa79ee0aac4a1a4d028577d8e0f5b298d25d8395"
        },
        "date": 1791165328237,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 196.68615080000222,
            "range": "± 2.632",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 100.7701040000029,
            "range": "± 4.8",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9518303841384996,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 241.55014660000234,
            "range": "± 4.764",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 268.5601527999978,
            "range": "± 11.951",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8994266054796656,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 192.75336119999622,
            "range": "± 2.58",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 169.46429259999718,
            "range": "± 8.835",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1374275857331813,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 101.60157020000042,
            "range": "± 1.561",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 102.56467680000014,
            "range": "± 2.539",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9906097632240605,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 98.5931608000044,
            "range": "± 1.349",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 57.28353000000084,
            "range": "± 1.947",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.721143246584192,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 208.39074220000384,
            "range": "± 2.664",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 255.32505519999802,
            "range": "± 4.828",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8161781930753714,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 305.6242566000009,
            "range": "± 8.408",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 234.8228913999975,
            "range": "± 11.83",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3015096389363516,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 151.33690780000393,
            "range": "± 1.988",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 128.69985639999584,
            "range": "± 2.19",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.175890261521759,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 140.38501240000016,
            "range": "± 2.748",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 129.5671532,
            "range": "± 7.086",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.08349229671892,
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
          "id": "d702c0da0733700e5f75cdf714ded46f8106cb72",
          "message": "Merge pull request #434 from patbuc/398-unify-size-contains\n\n398: Unify size and membership names: size, contains, isEmpty",
          "timestamp": "2026-10-05T08:06:41+02:00",
          "tree_id": "c9e42aabe5bf640b9b70d7092d052e2aef6e7c19",
          "url": "https://github.com/patbuc/neon/commit/d702c0da0733700e5f75cdf714ded46f8106cb72"
        },
        "date": 1791180479031,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 272.62424079999903,
            "range": "± 5.058",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 177.52171239999655,
            "range": "± 0.448",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.5357233609019891,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 311.43410420000066,
            "range": "± 2.086",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 502.9218109999988,
            "range": "± 21.841",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6192495481171355,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 264.5792116000024,
            "range": "± 0.58",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 302.7569028000073,
            "range": "± 5.818",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8738998488657944,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 144.67212880000204,
            "range": "± 0.585",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.33712039999955,
            "range": "± 5.926",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.760083626861481,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 129.05204779999622,
            "range": "± 0.997",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 90.45982740000227,
            "range": "± 1.687",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.4266227507747045,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 322.24475820000293,
            "range": "± 6.661",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 431.8812995999991,
            "range": "± 1.01",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.746141957288867,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 467.5312246000004,
            "range": "± 1.435",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 446.0097695999991,
            "range": "± 15.911",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0482533264222051,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 228.4045710000015,
            "range": "± 1.634",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 253.3861504000015,
            "range": "± 3.338",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.9014090574383664,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 198.01684920000184,
            "range": "± 0.281",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.43434959999888,
            "range": "± 12.114",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8002803552542922,
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
          "id": "bcae54649d47a26e00beeda68ffcffc3008ffaff",
          "message": "Merge pull request #435 from patbuc/399-indexable-strings\n\n399: Make strings indexable and iterable",
          "timestamp": "2026-10-05T08:34:09+02:00",
          "tree_id": "09a3bb60e36d197cf1fd9113509d4412e0d5400e",
          "url": "https://github.com/patbuc/neon/commit/bcae54649d47a26e00beeda68ffcffc3008ffaff"
        },
        "date": 1791182101538,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 196.745414599998,
            "range": "± 1.755",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 107.06218879999909,
            "range": "± 5.117",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8376741294495156,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 245.42412739999975,
            "range": "± 2.718",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 276.73390339999884,
            "range": "± 3.436",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8868596308030134,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 197.28884120000174,
            "range": "± 1.376",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 172.78759539999982,
            "range": "± 6.674",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1417997961212554,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 105.12457939999962,
            "range": "± 2.951",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 111.23029080000038,
            "range": "± 2.512",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9451074760653171,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 100.81378200000017,
            "range": "± 0.418",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 60.22644179999901,
            "range": "± 2.178",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6739123047445554,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 216.16946340000283,
            "range": "± 2.381",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 262.14821639999855,
            "range": "± 14.897",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8246077977130346,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 325.48022159999874,
            "range": "± 15.854",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 242.69626619999798,
            "range": "± 11.286",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3411010671741495,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 165.1467860000011,
            "range": "± 7.545",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 145.7798798000013,
            "range": "± 1.145",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1328503372795318,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 142.6900793999991,
            "range": "± 1.672",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 131.95643039999538,
            "range": "± 2.105",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.081342371625749,
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
          "id": "80d667d8e494e1677e1ed5cde18919a7aab0661e",
          "message": "Merge pull request #436 from patbuc/400-array-range-helpers\n\n400: Add forEach, flatMap, take, drop, first, last, chunked, zip, withIndex",
          "timestamp": "2026-10-05T09:19:50+02:00",
          "tree_id": "5e90d1d15ce6fbf3bed67d017bb43a610e6150a6",
          "url": "https://github.com/patbuc/neon/commit/80d667d8e494e1677e1ed5cde18919a7aab0661e"
        },
        "date": 1791184863507,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 375.45426580000054,
            "range": "± 0.895",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.23769359999807,
            "range": "± 1.532",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0052279996681417,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 439.17683880000027,
            "range": "± 0.967",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.2119245999952,
            "range": "± 17.58",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8267450681396172,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 356.94025039999815,
            "range": "± 6.318",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.3557338000021,
            "range": "± 7.901",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1464065429072106,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 187.16121140000155,
            "range": "± 0.658",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 196.02762380000627,
            "range": "± 10.554",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9547695767151138,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.40686839999955,
            "range": "± 1.804",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.9168594000048,
            "range": "± 0.62",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6322051901711425,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 394.02206499999863,
            "range": "± 2.198",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 439.08183840000277,
            "range": "± 2.093",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8973772780851055,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 540.7815234000026,
            "range": "± 1.356",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 429.18998160000115,
            "range": "± 31.299",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2600050014774182,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 258.8953308000015,
            "range": "± 2.852",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 228.6053743999986,
            "range": "± 17.041",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1324988814436336,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 250.72254060000319,
            "range": "± 1.822",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.95246999999915,
            "range": "± 6.492",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.040547708848986,
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
          "id": "5f19faeee6c09e46d8ad71f30cf0a35e63b75b41",
          "message": "Merge pull request #437 from patbuc/401-sort-group-helpers\n\n401: Add sortBy, minBy, maxBy, groupBy, tally",
          "timestamp": "2026-10-05T10:03:38+02:00",
          "tree_id": "b2a0bf2b3353fb8efbfe2cf825305f277efa406c",
          "url": "https://github.com/patbuc/neon/commit/5f19faeee6c09e46d8ad71f30cf0a35e63b75b41"
        },
        "date": 1791187498053,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 339.16514879999795,
            "range": "± 2.541",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 195.80987920001007,
            "range": "± 0.842",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.732114590875967,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 443.96335280000017,
            "range": "± 1.785",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 636.6462703999957,
            "range": "± 10.776",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6973469781281595,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 348.61217480000164,
            "range": "± 0.355",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 362.3408394000023,
            "range": "± 11.898",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9621111861894069,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 189.5525235999969,
            "range": "± 0.958",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 216.35182079999709,
            "range": "± 8.41",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8761309375585318,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 182.73558499999467,
            "range": "± 1.695",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.00068539999961,
            "range": "± 1.346",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6315577386635824,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 371.98906939999006,
            "range": "± 10.316",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 464.69019620000154,
            "range": "± 11.772",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.800509828789,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 582.9753930000038,
            "range": "± 4.139",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 491.2596841999914,
            "range": "± 25.576",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1866949634781654,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 288.28556799999774,
            "range": "± 43.131",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.07211759998927,
            "range": "± 3.572",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.2422240594059675,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.4671387999981,
            "range": "± 1.167",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 264.41748659999575,
            "range": "± 14.808",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9396774093684396,
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
          "id": "7ff9f24f47cff8fa40f12000e1108a2b2a699fdd",
          "message": "Merge pull request #438 from patbuc/402-prep-stack-height\n\n402: Track the operand stack height in codegen",
          "timestamp": "2026-10-05T11:05:26+02:00",
          "tree_id": "67a4beb306b5dfd596c0fae95058b2d7b9f6216f",
          "url": "https://github.com/patbuc/neon/commit/7ff9f24f47cff8fa40f12000e1108a2b2a699fdd"
        },
        "date": 1791191206411,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 373.6128484000034,
            "range": "± 2.086",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 191.1085660000026,
            "range": "± 0.685",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.954976986222576,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 445.0457035999989,
            "range": "± 6.241",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 552.979154800002,
            "range": "± 43.381",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8048146114313481,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 354.94557959999895,
            "range": "± 0.586",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 305.76504919999934,
            "range": "± 3.041",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1608441858501326,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 186.91166260000216,
            "range": "± 0.428",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.54676099999983,
            "range": "± 5.113",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9707338707193435,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.1855274000037,
            "range": "± 0.654",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.04685339999969,
            "range": "± 2.59",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.590276942127892,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 389.39226620000227,
            "range": "± 2.425",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 434.43376620000436,
            "range": "± 3.989",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8963213647180779,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 543.6816620000002,
            "range": "± 8.585",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.3958009999965,
            "range": "± 10.747",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3378131876908956,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 254.29573680000033,
            "range": "± 1.528",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 216.76001079999878,
            "range": "± 1.252",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1731672085707505,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 245.94760160000249,
            "range": "± 1.062",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 248.25867500000243,
            "range": "± 14.296",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9906908654853656,
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
          "id": "ac75176e7527e4e2c0860402c5b6bb72f9957df3",
          "message": "Merge pull request #439 from patbuc/402-if-expression\n\n402: Make if/else an expression",
          "timestamp": "2026-10-05T12:05:04+02:00",
          "tree_id": "9e7288d740a815bb61aedd3894252cc84c63b599",
          "url": "https://github.com/patbuc/neon/commit/ac75176e7527e4e2c0860402c5b6bb72f9957df3"
        },
        "date": 1791194782388,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 294.7392513999972,
            "range": "± 30.447",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 143.32031399999892,
            "range": "± 0.914",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0565071564104964,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 351.4659635999976,
            "range": "± 0.434",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 469.27065500000253,
            "range": "± 27.401",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.748962160440217,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 278.2743052000001,
            "range": "± 0.946",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 270.016749200002,
            "range": "± 9.893",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0305816436367867,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 151.91498479999268,
            "range": "± 0.57",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 159.73903280000457,
            "range": "± 3.782",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9510198111077354,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 140.33016059999852,
            "range": "± 0.366",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.23730019999766,
            "range": "± 0.713",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7710113828436909,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 315.4890522000045,
            "range": "± 0.915",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 391.19784779999804,
            "range": "± 2.097",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8064692941800128,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 452.50089819999744,
            "range": "± 5.48",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 350.1261614000015,
            "range": "± 3.565",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.292393851378155,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 217.2316037999991,
            "range": "± 1.44",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 209.39561300000094,
            "range": "± 2.845",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.03742194350556,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 204.0427013999988,
            "range": "± 3.318",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 216.1983964000001,
            "range": "± 2.663",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9437752767716584,
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
          "id": "d8a52d9e7aed87902e38da3cc4b27a6ab03f70ac",
          "message": "Merge pull request #440 from patbuc/403-trailing-blocks\n\n403: Add trailing-block lambdas with it",
          "timestamp": "2026-10-05T14:08:50+02:00",
          "tree_id": "dcd329c99acbadedcb9bcc0118d57e4af6c8a869",
          "url": "https://github.com/patbuc/neon/commit/d8a52d9e7aed87902e38da3cc4b27a6ab03f70ac"
        },
        "date": 1791202207449,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 257.6370620000034,
            "range": "± 4.872",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 129.25198319994706,
            "range": "± 1.131",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9932929121981082,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 317.9063232000317,
            "range": "± 3.047",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 406.9473532000302,
            "range": "± 4.502",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7811976677085514,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 250.30085320008766,
            "range": "± 1.601",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 230.93202040004144,
            "range": "± 4.34",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0838724433558142,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 136.23372239999298,
            "range": "± 0.9",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 146.21433599991178,
            "range": "± 4.674",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9317398425286778,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 126.03615340008218,
            "range": "± 1.01",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 73.27233880000676,
            "range": "± 4.397",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7201055059002777,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 285.87410800000725,
            "range": "± 0.853",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 364.1141831999903,
            "range": "± 2.757",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7851221435199915,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 453.3103797999047,
            "range": "± 10.752",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 328.32616779996897,
            "range": "± 10.031",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3806708823650076,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 203.24772760000087,
            "range": "± 2.95",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 195.79507879998346,
            "range": "± 4.482",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0380635143931822,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 179.92323379999107,
            "range": "± 1.714",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 193.84635279993745,
            "range": "± 3.281",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9281744598294507,
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
          "id": "dc95fc081b0dc06455679705dfff15801b8fe65e",
          "message": "Merge pull request #441 from patbuc/404-tuple-destructuring\n\n404: Destructure tuples in val/var and for",
          "timestamp": "2026-10-05T15:32:35+02:00",
          "tree_id": "77a84d8a4d0a7a196dd60628ac76261a51d9f95c",
          "url": "https://github.com/patbuc/neon/commit/dc95fc081b0dc06455679705dfff15801b8fe65e"
        },
        "date": 1791207211368,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 205.281166400016,
            "range": "± 2.648",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 105.07386499999711,
            "range": "± 1.175",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9536843571902645,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 254.04825939999682,
            "range": "± 1.192",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 286.19405319999487,
            "range": "± 10.027",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8876783306970593,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 205.92643679999583,
            "range": "± 2.97",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 174.7064211999941,
            "range": "± 9.444",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.178699874827513,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 106.77413680000427,
            "range": "± 0.532",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 112.82812100000683,
            "range": "± 5.002",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9463433039002556,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 101.19747139999618,
            "range": "± 0.339",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 57.36789800000679,
            "range": "± 0.28",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7640087039616512,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 217.50731759998416,
            "range": "± 4.326",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 256.71483820000276,
            "range": "± 5.4",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.847272090406116,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 305.98376919999737,
            "range": "± 5.435",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 252.3266408000154,
            "range": "± 18.557",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2126494777953651,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 157.88226460000487,
            "range": "± 1.019",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 136.74594419999266,
            "range": "± 5.318",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1545663421585657,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 150.45022819999758,
            "range": "± 1.362",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 141.36983099999725,
            "range": "± 6.984",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0642315063671577,
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
          "id": "a75419c2466d2e809b4be96e5cbbd9b98dd940cf",
          "message": "Merge pull request #442 from patbuc/405-match-expressions\n\n405: Add match expressions",
          "timestamp": "2026-10-05T16:33:17+02:00",
          "tree_id": "25a0b4db246f5535a71fbcc675a189e64cd5d887",
          "url": "https://github.com/patbuc/neon/commit/a75419c2466d2e809b4be96e5cbbd9b98dd940cf"
        },
        "date": 1791210879889,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 374.4124477999975,
            "range": "± 0.491",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.74040859999468,
            "range": "± 1.09",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9837429121683487,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 438.36653380000143,
            "range": "± 0.47",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.516733999996,
            "range": "± 12.699",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8247464393096695,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.7950540000006,
            "range": "± 0.314",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.87143599999945,
            "range": "± 12.444",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.124011342019671,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 188.81342999999902,
            "range": "± 2.167",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 193.03409820000184,
            "range": "± 6.773",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9781351158196424,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.54669719999947,
            "range": "± 0.253",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.52190980000114,
            "range": "± 2.182",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6010010725264467,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.5885423999972,
            "range": "± 1.172",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 443.91606980000233,
            "range": "± 2.542",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8843756041020778,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 541.4013699999941,
            "range": "± 0.621",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 411.5022800000048,
            "range": "± 23.905",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3156704016317669,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 267.89688180000155,
            "range": "± 1.612",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 231.39859979999926,
            "range": "± 2.968",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1577290529482382,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 252.34302260000163,
            "range": "± 2.306",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.36742580000248,
            "range": "± 5.787",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0368808470176087,
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
          "id": "edc85a86b1976368747df62cf82113ac745abaa6",
          "message": "Merge pull request #447 from patbuc/384-scripts-else-if-short-circuit\n\n384: Add scripts for else-if, short-circuit, negative index, and empty pop",
          "timestamp": "2026-10-05T17:31:28+02:00",
          "tree_id": "aa5fcd0d1b9da42291f8f7ef002b6a59d4f0f464",
          "url": "https://github.com/patbuc/neon/commit/edc85a86b1976368747df62cf82113ac745abaa6"
        },
        "date": 1791214364575,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 357.34467079999774,
            "range": "± 35.16",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 197.8655764000024,
            "range": "± 5.804",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8059971688940706,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 441.84372759999917,
            "range": "± 2.288",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 635.0152909999963,
            "range": "± 7.788",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6958001387717792,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 390.9994330000018,
            "range": "± 93.468",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 363.3861018000033,
            "range": "± 5.755",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0759889579244173,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 190.2730312000017,
            "range": "± 1.366",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 221.59946099999672,
            "range": "± 4.762",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.858634900741047,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 183.15981739999927,
            "range": "± 2.535",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 114.94069839999668,
            "range": "± 3.677",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.593515786397941,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 369.7245672000008,
            "range": "± 4.003",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 469.26292939999996,
            "range": "± 6.439",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7878836022115171,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 605.1680809999965,
            "range": "± 14.713",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 498.1558421999921,
            "range": "± 34.324",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2148167897166662,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 272.0195627999942,
            "range": "± 1.338",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 240.0928024000052,
            "range": "± 5.515",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1329767493271106,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 257.1448116000113,
            "range": "± 4.82",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 271.24966880000443,
            "range": "± 3.87",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9480004629594854,
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
          "id": "492f8d5cf42d7b2bce574c22633a79845e58311f",
          "message": "Merge pull request #448 from patbuc/377-prune-basic-rs-duplicates\n\n377: Remove basic.rs VM tests covered by scripts or stdlib tests",
          "timestamp": "2026-10-05T18:27:04+02:00",
          "tree_id": "a1116b2ccfd7d062354f7963ca483b1c066eb57d",
          "url": "https://github.com/patbuc/neon/commit/492f8d5cf42d7b2bce574c22633a79845e58311f"
        },
        "date": 1791217696248,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 264.8876522000023,
            "range": "± 1.348",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 152.33610120000378,
            "range": "± 1.332",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7388370196781415,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 342.5752643999999,
            "range": "± 0.418",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 480.31574539999724,
            "range": "± 46.374",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7132293031841173,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 271.36564360000364,
            "range": "± 0.644",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 277.7767183999998,
            "range": "± 10.471",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9769200426985958,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 147.29923000000156,
            "range": "± 0.286",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 171.27235940000105,
            "range": "± 8.638",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.860029198616859,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 140.98171420000085,
            "range": "± 0.649",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.65191980000395,
            "range": "± 1.303",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6084269976251506,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 288.0201495999984,
            "range": "± 3.566",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 360.66183320000107,
            "range": "± 9.282",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7985878268418826,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 451.5624957999961,
            "range": "± 8.816",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 372.8833411999972,
            "range": "± 15.068",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2110020639345191,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 212.10562320000008,
            "range": "± 1.402",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 189.90741220000302,
            "range": "± 3.22",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1168896502924213,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 196.30967319999968,
            "range": "± 2.513",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 207.90941540000176,
            "range": "± 7.902",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9442077109510165,
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
          "id": "a435b340545d6c35004fb19b9f72b98681289669",
          "message": "Merge pull request #449 from patbuc/378-prune-weak-stdlib-tests\n\n378: Remove stdlib unit tests weaker than their scripts",
          "timestamp": "2026-10-05T18:42:38+02:00",
          "tree_id": "61fa318067de36cd94a85fefa4aa06050ea873eb",
          "url": "https://github.com/patbuc/neon/commit/a435b340545d6c35004fb19b9f72b98681289669"
        },
        "date": 1791218636181,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 402.20313140000314,
            "range": "± 65.799",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.0114442000007,
            "range": "± 2.963",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.139248135194111,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 441.5405318000012,
            "range": "± 8.31",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 540.5354507999959,
            "range": "± 13.98",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8168576753782169,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.7384222000052,
            "range": "± 1.079",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 313.7671993999959,
            "range": "± 10.015",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1242042599562108,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 187.11359939999852,
            "range": "± 0.369",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 193.14322199999765,
            "range": "± 6.607",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.968781598766126,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.9365310000003,
            "range": "± 0.539",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.58611420000182,
            "range": "± 0.535",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6237142114123555,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 391.8134637999998,
            "range": "± 3.802",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 432.25749939999787,
            "range": "± 2.501",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.9064353177072992,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 540.6052841999951,
            "range": "± 1.707",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 408.0180197999965,
            "range": "± 13.689",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.324954433299271,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 261.7461169999956,
            "range": "± 2.836",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 222.62626800000191,
            "range": "± 14.96",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1757198256586385,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 254.55101399999478,
            "range": "± 1.679",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 247.81836600000418,
            "range": "± 10.745",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0271676716647808,
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
          "id": "6f840522a3ad110ba7a484b97146c221acc68c5a",
          "message": "Merge pull request #450 from patbuc/379-table-drive-math-errors\n\n379: Table-drive math error tests on the exact message",
          "timestamp": "2026-10-05T18:48:30+02:00",
          "tree_id": "5f4d6694128ce2c6a5e855518fbaa8d7a2e08180",
          "url": "https://github.com/patbuc/neon/commit/6f840522a3ad110ba7a484b97146c221acc68c5a"
        },
        "date": 1791218985562,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 194.97922580000022,
            "range": "± 2.037",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 104.95308459999819,
            "range": "± 1.708",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.857775086297974,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 246.4860974000004,
            "range": "± 0.459",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 281.2011336000012,
            "range": "± 10.939",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8765473106186631,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 199.93884719999642,
            "range": "± 0.681",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 178.59319039999377,
            "range": "± 8.713",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.119521112491439,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 106.31680199999778,
            "range": "± 2.877",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 110.64114240000151,
            "range": "± 5.342",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9609156204807618,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 100.84665060000191,
            "range": "± 0.163",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 60.14316159999851,
            "range": "± 0.475",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6767766761367664,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 219.0398531999989,
            "range": "± 2.388",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 261.0006418000012,
            "range": "± 1.506",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8392310903505139,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 316.41802640000094,
            "range": "± 3.1",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 257.96611799999596,
            "range": "± 21.822",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2265875412367369,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 170.97644759999753,
            "range": "± 1.925",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 152.0764593999985,
            "range": "± 5.675",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.124279512256972,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 143.68665199999668,
            "range": "± 1.23",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 139.27394720000166,
            "range": "± 12.39",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0316836342238382,
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
          "id": "915eb7ee848dc752f35b1480562f835e572b6567",
          "message": "Merge pull request #451 from patbuc/380-merge-file-tests\n\n380: Merge the File unit tests into file_io_test.rs",
          "timestamp": "2026-10-05T18:57:07+02:00",
          "tree_id": "6dc829bde3bcffa981ec188e30aaa91165c42c28",
          "url": "https://github.com/patbuc/neon/commit/915eb7ee848dc752f35b1480562f835e572b6567"
        },
        "date": 1791219512242,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 375.0798894000013,
            "range": "± 1.447",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.47500560000208,
            "range": "± 1.184",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9795744996140916,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 438.6624039999987,
            "range": "± 0.94",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.2944500000015,
            "range": "± 8.282",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8256483838669827,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 352.5893800000006,
            "range": "± 0.275",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.1047846,
            "range": "± 9.391",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1370007736410803,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 187.95229819999975,
            "range": "± 1.199",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 191.7452204,
            "range": "± 1.714",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9802189478721408,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.53507580000212,
            "range": "± 0.213",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.2871589999969,
            "range": "± 0.847",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6042738210255425,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 394.59712899999886,
            "range": "± 1.901",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 443.5851040000017,
            "range": "± 3.773",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8895635255596801,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 541.2506757999978,
            "range": "± 1.106",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 411.3033013999967,
            "range": "± 15.013",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3159405090056058,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 275.85394519999795,
            "range": "± 4.578",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 239.9649032000042,
            "range": "± 26.011",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1495595460894596,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 251.00079980000203,
            "range": "± 2.165",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 239.27933580000058,
            "range": "± 5.476",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0489865284890239,
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
          "id": "5074b46869cbff0e8e8a1d947f3e8acb4e615c81",
          "message": "Merge pull request #452 from patbuc/381-remove-smoke-codegen-tests\n\n381: Remove smoke-only codegen tests and walk bytecode one way",
          "timestamp": "2026-10-05T19:08:26+02:00",
          "tree_id": "2f418bd741dbf17da3038c3f89fb9ce3cca5d8de",
          "url": "https://github.com/patbuc/neon/commit/5074b46869cbff0e8e8a1d947f3e8acb4e615c81"
        },
        "date": 1791220188045,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 377.52285340000356,
            "range": "± 8.326",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.26692520000177,
            "range": "± 1.129",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.026784159316761,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 438.4286944000024,
            "range": "± 1.024",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.6942160000025,
            "range": "± 9.191",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8245880455468418,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 351.7304143999979,
            "range": "± 0.415",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 321.80630219999955,
            "range": "± 11.243",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0929879619989569,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 187.0758350000017,
            "range": "± 0.961",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.9094521999965,
            "range": "± 1.826",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9902936714990122,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.58230800000092,
            "range": "± 0.325",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.61461619999966,
            "range": "± 2.864",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.620060482408563,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 392.0915968000003,
            "range": "± 2.434",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 435.9666097999991,
            "range": "± 1.797",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8993615290397432,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 538.5004855999995,
            "range": "± 1.219",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 460.1560219999982,
            "range": "± 130.859",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1702563040672358,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 261.07345460000033,
            "range": "± 0.742",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 218.53231739999615,
            "range": "± 3.93",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.194667487656473,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 255.45447419999903,
            "range": "± 2.441",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 244.55425440000624,
            "range": "± 5.344",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0445717856217043,
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
          "id": "d18e068d582d07c4f12d2e89b0cf2e26afc24de3",
          "message": "Merge pull request #453 from patbuc/382-table-drive-scanner-tests\n\n382: Iterate KEYWORDS and table-drive the scanner literal tests",
          "timestamp": "2026-10-05T19:18:37+02:00",
          "tree_id": "66ced1b3fda47468387b7b3690fcd45d13856e75",
          "url": "https://github.com/patbuc/neon/commit/d18e068d582d07c4f12d2e89b0cf2e26afc24de3"
        },
        "date": 1791220799262,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 273.0807659999982,
            "range": "± 8.032",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 177.64375499999971,
            "range": "± 0.523",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.537238199001134,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 309.52380420000054,
            "range": "± 0.627",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 491.6723063999967,
            "range": "± 16.826",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6295327196000124,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 260.4623404000023,
            "range": "± 2.372",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 301.01325859999974,
            "range": "± 3.451",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8652852755104605,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 142.5336953999988,
            "range": "± 1.185",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 187.88003639999715,
            "range": "± 6.378",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.7586420469737623,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 126.86906219999514,
            "range": "± 0.682",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 90.5684674000014,
            "range": "± 1.531",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.4008083148814903,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 334.0559227999961,
            "range": "± 4.164",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 435.5688561999983,
            "range": "± 10.601",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7669417086298957,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 461.06378880000136,
            "range": "± 2.594",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 453.15352059999725,
            "range": "± 19.818",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0174560448951837,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 226.70738700000186,
            "range": "± 4.156",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 253.95591720000255,
            "range": "± 2.686",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.8927037003097622,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 201.96709200000384,
            "range": "± 4.174",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 256.13302019999935,
            "range": "± 10.781",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.7885242279277366,
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
          "id": "af5a8766bac84ffbcf6abad9bb9f1a38acdb1c67",
          "message": "Merge pull request #454 from patbuc/383-share-compiler-test-helpers\n\n383: Share compile and error helpers across compiler tests",
          "timestamp": "2026-10-05T19:40:07+02:00",
          "tree_id": "c04e31dba1d271c0b4cfc9b979c7ae4b3d6e5f3b",
          "url": "https://github.com/patbuc/neon/commit/af5a8766bac84ffbcf6abad9bb9f1a38acdb1c67"
        },
        "date": 1791222089788,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 374.83563199999423,
            "range": "± 0.689",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.88364639999702,
            "range": "± 4.09",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.974027985592761,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 443.70366219999653,
            "range": "± 8.1",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 538.7899863999991,
            "range": "± 15.15",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8235187612981912,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 353.04075860000523,
            "range": "± 0.677",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 311.29248259999827,
            "range": "± 6.574",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1341127021485202,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 187.57143039999846,
            "range": "± 0.278",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 195.3960885999976,
            "range": "± 11.458",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9599548882679158,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.6633088000002,
            "range": "± 0.217",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.82331600000168,
            "range": "± 0.447",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5977285881953056,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 395.02531219999923,
            "range": "± 4.625",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 449.07069960000285,
            "range": "± 3.213",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8796506041294989,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 542.6685117999966,
            "range": "± 3.074",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 412.4777912000013,
            "range": "± 13.376",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3156308615337515,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 285.97295079999583,
            "range": "± 2.016",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 250.82250659999943,
            "range": "± 8.108",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1401407101638321,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 252.07471500000054,
            "range": "± 3.051",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.81974540000147,
            "range": "± 4.676",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0254453505751577,
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
          "id": "614963f8b6567a69d41927229b5d2e8b1dcea843",
          "message": "Merge pull request #467 from patbuc/456-bind-names-guards-match-arms\n\n456: Bind names and add guards in match arms",
          "timestamp": "2026-10-05T23:34:28+02:00",
          "tree_id": "6dd927b576ac32638673f57ea9b9d70006b3ff99",
          "url": "https://github.com/patbuc/neon/commit/614963f8b6567a69d41927229b5d2e8b1dcea843"
        },
        "date": 1791236190272,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 372.33751480000024,
            "range": "± 2.276",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.7143065999993,
            "range": "± 3.033",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9730221916307091,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 441.65620900000135,
            "range": "± 0.765",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 535.7108998000001,
            "range": "± 14.036",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8244301341729042,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 356.0964909999967,
            "range": "± 3.718",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.15844920000006,
            "range": "± 5.16",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.1481115278931975,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 192.1226809999979,
            "range": "± 3.149",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 194.6680667999999,
            "range": "± 8.29",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9869244820589033,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.13997500000198,
            "range": "± 0.141",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 114.708302599999,
            "range": "± 6.078",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5529823993751917,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 405.89777039999717,
            "range": "± 37.205",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 440.4470695999976,
            "range": "± 2.115",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.9215585672272104,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 544.2069912000022,
            "range": "± 4.807",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.95260740000026,
            "range": "± 10.997",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.3146601361400239,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 265.8621737999965,
            "range": "± 1.677",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.80209619999823,
            "range": "± 7.329",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1420093639173965,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 248.59161479999443,
            "range": "± 2.191",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 250.81014160001018,
            "range": "± 12.882",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9911545570451699,
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
          "id": "1c48da921d54cc809ce32f04c9368553ad5b26b0",
          "message": "Merge pull request #468 from patbuc/427-tune-release-profile\n\n427: Tune the release profile",
          "timestamp": "2026-10-05T23:38:23+02:00",
          "tree_id": "6a834850ba175642171b6487900ca8ceafb8f62b",
          "url": "https://github.com/patbuc/neon/commit/1c48da921d54cc809ce32f04c9368553ad5b26b0"
        },
        "date": 1791236380888,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 241.58092760000045,
            "range": "± 0.85",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 151.50835939999183,
            "range": "± 0.498",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.5945056005933722,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 331.1377153999956,
            "range": "± 0.349",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 481.4239530000009,
            "range": "± 41.786",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6878297461862996,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 244.9125896000055,
            "range": "± 0.799",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 288.1793549999941,
            "range": "± 11.182",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8498616758997554,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 139.58374499999877,
            "range": "± 0.365",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 173.48238080000442,
            "range": "± 3.811",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8045989705485712,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.6961927999996,
            "range": "± 0.138",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.8761489999979,
            "range": "± 1.942",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5718074463375198,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 252.26992459998883,
            "range": "± 4.242",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 363.88294239999937,
            "range": "± 6.511",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6932721906010103,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 401.59910099999934,
            "range": "± 10.411",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 372.8883107999934,
            "range": "± 9.78",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0769956830730625,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 207.13656940000078,
            "range": "± 4.878",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 189.2499695999902,
            "range": "± 5.633",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0945130920645152,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 182.8650122000056,
            "range": "± 2.398",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 215.2633556000012,
            "range": "± 6.589",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8494943864937343,
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
          "id": "3c627c060814e13fdce90650f0bc609c55fb9384",
          "message": "Merge pull request #469 from patbuc/457-match-array-patterns\n\n457: Match arrays with element and rest patterns",
          "timestamp": "2026-10-05T23:56:50+02:00",
          "tree_id": "acd23372ba65370ae5d9ec13c21d40cbe6f238a1",
          "url": "https://github.com/patbuc/neon/commit/3c627c060814e13fdce90650f0bc609c55fb9384"
        },
        "date": 1791237507217,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 338.88991559999795,
            "range": "± 1.51",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.62391160000038,
            "range": "± 3.791",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7871686789947923,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 443.2078861999969,
            "range": "± 44.735",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 529.7609969999996,
            "range": "± 7.501",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8366185670705335,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 315.7817085999909,
            "range": "± 0.767",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 309.09599120000166,
            "range": "± 5.905",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0216299065349677,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 177.60551960000157,
            "range": "± 0.663",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 202.98193940000147,
            "range": "± 9.81",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8749818832403977,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 176.86551219999842,
            "range": "± 0.14",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.27071180000212,
            "range": "± 0.17",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.618599433338687,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 340.20697000000837,
            "range": "± 5.296",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.1347806000074,
            "range": "± 6.47",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7782656176043536,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 516.6411837999988,
            "range": "± 1.15",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 409.1829232000009,
            "range": "± 15.983",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.262616679502713,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 246.73096120000082,
            "range": "± 2.047",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 220.9884785999975,
            "range": "± 5.117",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1164878945865715,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 241.0686989999988,
            "range": "± 9.823",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.7847883999989,
            "range": "± 10.704",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9808121184768971,
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
          "id": "ad9db13d599d2fd9e39bf0a05a1a49114f1172c1",
          "message": "Merge pull request #470 from patbuc/worktree-460-writing-neon-new-syntax\n\n460: Close remaining gaps in the writing-neon skill",
          "timestamp": "2026-10-06T00:17:40+02:00",
          "tree_id": "d78e950ff2ac92e4d8395f708fb486211c9e9c2c",
          "url": "https://github.com/patbuc/neon/commit/ad9db13d599d2fd9e39bf0a05a1a49114f1172c1"
        },
        "date": 1791238761209,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 240.9440434000004,
            "range": "± 2.512",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 132.6792666000017,
            "range": "± 3.018",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8159886587735878,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 310.129521799999,
            "range": "± 13.906",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 412.14618780000194,
            "range": "± 3.896",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7524745611634589,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 230.87779659999512,
            "range": "± 4.566",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 240.30883599999981,
            "range": "± 7.932",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9607545042579928,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 135.24416680000115,
            "range": "± 2.623",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 149.61073600000248,
            "range": "± 3.464",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.903973674723443,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 130.05542919999584,
            "range": "± 0.844",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 72.61766459999706,
            "range": "± 1.983",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7909613303648038,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 253.25732100000096,
            "range": "± 6.65",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 352.1948597999966,
            "range": "± 5.539",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7190829563606351,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 368.6256470000018,
            "range": "± 4.42",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 313.5843268000002,
            "range": "± 2.194",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1755231862563915,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 181.26769439999748,
            "range": "± 1.057",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 177.76504439999314,
            "range": "± 4.343",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0197038175408826,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 175.7518444000027,
            "range": "± 3.058",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 186.86327540000036,
            "range": "± 3.617",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.940537106736396,
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
          "id": "73e57f4c0db953c626310ccbe7c2fbd75bd6c0a6",
          "message": "Merge pull request #471 from patbuc/458-enum-payload-fields\n\n458: Add payload fields to enum variants",
          "timestamp": "2026-10-06T00:38:25+02:00",
          "tree_id": "7a8773c7b66f07a8b98576967d62481b42a47761",
          "url": "https://github.com/patbuc/neon/commit/73e57f4c0db953c626310ccbe7c2fbd75bd6c0a6"
        },
        "date": 1791240002811,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 347.8265936000014,
            "range": "± 21.417",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 190.8959748000001,
            "range": "± 4.124",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8220740063504013,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 417.7695629999988,
            "range": "± 1.079",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 527.8005368000009,
            "range": "± 11.444",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.791529249918713,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 318.48033560000033,
            "range": "± 1.159",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 307.8827587999996,
            "range": "± 9.059",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0344208192797342,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 179.0009084000019,
            "range": "± 1.783",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 188.86897959999942,
            "range": "± 2.277",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9477517630428414,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.07964260000233,
            "range": "± 0.519",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.74640419999662,
            "range": "± 2.815",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5936051264905733,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 345.1958311999988,
            "range": "± 3.777",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 448.4925067999967,
            "range": "± 18.175",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7696802643660162,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 516.2814416000032,
            "range": "± 1.739",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 417.33195299999863,
            "range": "± 26.032",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2371001977890843,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 273.2719492000058,
            "range": "± 38.431",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.52664400000072,
            "range": "± 32.792",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1752285437018777,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 239.14435639999851,
            "range": "± 4.838",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 246.61304820000112,
            "range": "± 17.352",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9697149366000068,
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
          "id": "c66ef27af40d9abf2a98d6d3c056013d998c4369",
          "message": "Merge pull request #472 from patbuc/459-match-enum-variant-fields\n\n459: Match enum variants and bind their fields",
          "timestamp": "2026-10-06T00:59:19+02:00",
          "tree_id": "9baa8fb7b7f73e45e80327569f239fafa1b97b33",
          "url": "https://github.com/patbuc/neon/commit/c66ef27af40d9abf2a98d6d3c056013d998c4369"
        },
        "date": 1791241263092,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 262.6041891999989,
            "range": "± 0.859",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 145.13059599998996,
            "range": "± 1.916",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8094336855063773,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 337.9677097999888,
            "range": "± 1.237",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 464.0086943999904,
            "range": "± 9.494",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7283650368599555,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 254.95686839999507,
            "range": "± 1.813",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 258.8657903999888,
            "range": "± 1.648",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9848998123933108,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 146.45190480001702,
            "range": "± 0.39",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 163.80873359996713,
            "range": "± 3.817",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8940421037480409,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 141.30528580000146,
            "range": "± 0.547",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 79.52893480000967,
            "range": "± 3.261",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.776778302832044,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 283.8721841999927,
            "range": "± 1.277",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 399.1090729999996,
            "range": "± 2.306",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7112646727527364,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 468.51163160000624,
            "range": "± 13.351",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 364.21159699999635,
            "range": "± 11.398",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2863720855105307,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 222.5032530000135,
            "range": "± 1.975",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 218.40990479998936,
            "range": "± 6.306",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0187415868514418,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 194.70074299998714,
            "range": "± 0.574",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 223.25761060000104,
            "range": "± 13.113",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8720900598941832,
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
          "id": "1a1801260ed9f20118e4cbbcee38918e856d4276",
          "message": "Merge pull request #473 from patbuc/461-tail-calls\n\n461: Add tail calls",
          "timestamp": "2026-10-06T01:41:12+02:00",
          "tree_id": "b7da3e1bcf033031dc6f5332dc4d5948f5f389e0",
          "url": "https://github.com/patbuc/neon/commit/1a1801260ed9f20118e4cbbcee38918e856d4276"
        },
        "date": 1791243767145,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 256.75523279999766,
            "range": "± 12.012",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 178.3556367999978,
            "range": "± 1.356",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.4395689276022974,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 304.8322634000016,
            "range": "± 2.847",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 493.04043179999724,
            "range": "± 15.868",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6182703156556891,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 235.00082259999715,
            "range": "± 1.716",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 305.1458681999975,
            "range": "± 4.807",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.7701261825573003,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 138.23149119999982,
            "range": "± 1.497",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 184.21420379999915,
            "range": "± 2.925",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.7503845433660337,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 128.57126280000273,
            "range": "± 1.155",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 92.02043979999814,
            "range": "± 5.548",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.3972033069983798,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 285.8890758000001,
            "range": "± 5.711",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 430.1760311999999,
            "range": "± 1.743",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6645862508947713,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 433.56741300000294,
            "range": "± 16.898",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 440.77918960000204,
            "range": "± 27.483",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 0.9836385728497218,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 220.4176906000015,
            "range": "± 10.873",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 256.8490321999974,
            "range": "± 16.09",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.8581604871626366,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 193.36463360000096,
            "range": "± 2.051",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.30227400000228,
            "range": "± 8",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.7882708563883887,
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
          "id": "962fc023e0a533c6ac4310a90b26a6cdab88d150",
          "message": "Merge pull request #474 from patbuc/463-map-higher-order-methods\n\n463: Add higher-order methods to Map",
          "timestamp": "2026-10-06T01:49:56+02:00",
          "tree_id": "b25ef1d71be41323e8fbaa4b89620936aecb04b8",
          "url": "https://github.com/patbuc/neon/commit/962fc023e0a533c6ac4310a90b26a6cdab88d150"
        },
        "date": 1791244284685,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 187.5972020000006,
            "range": "± 4.643",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 105.34306559999891,
            "range": "± 3.567",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7808215560417728,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 234.1093189999981,
            "range": "± 7.063",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 277.134917799998,
            "range": "± 12.59",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8447485465146239,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 175.36495360000117,
            "range": "± 4.019",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 171.09620339999765,
            "range": "± 2.686",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0249494150961598,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 105.25357760000702,
            "range": "± 2.52",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 105.77186419999407,
            "range": "± 1.428",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9950999577826569,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 98.53089159999513,
            "range": "± 0.702",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 59.44586779999099,
            "range": "± 0.702",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6574893301500437,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 188.27290860000403,
            "range": "± 1.354",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 252.30145859998743,
            "range": "± 2.555",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7462220378935749,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 278.4612311999979,
            "range": "± 13.459",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 234.79152019999674,
            "range": "± 13.061",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1859935612785464,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 147.81024779999825,
            "range": "± 2.896",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 132.33618779999006,
            "range": "± 3.648",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1169299211142105,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 143.54345440000316,
            "range": "± 20.283",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 129.6674535999955,
            "range": "± 2.571",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.107012210194341,
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
          "id": "9ebeabf9696abf01e031a2fffc587b49623d572f",
          "message": "Merge pull request #475 from patbuc/464-takewhile-dropwhile-partition\n\n464: Add takeWhile, dropWhile and partition to Array and Range",
          "timestamp": "2026-10-06T02:00:04+02:00",
          "tree_id": "958e6fe894b94bee8b851e52f5ee625715b79ea6",
          "url": "https://github.com/patbuc/neon/commit/9ebeabf9696abf01e031a2fffc587b49623d572f"
        },
        "date": 1791244913723,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 249.93695999999943,
            "range": "± 0.573",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 177.87102320000372,
            "range": "± 1.161",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.4051583866977733,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 300.5067268000033,
            "range": "± 2.148",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 493.61915659999624,
            "range": "± 8.778",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6087825457785436,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 236.4943065999995,
            "range": "± 2.892",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 302.0192849999944,
            "range": "± 5.518",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.7830437271580319,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 139.76925620000316,
            "range": "± 1.658",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 185.4869882000031,
            "range": "± 4.291",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.7535259349259347,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 129.00356939999824,
            "range": "± 0.643",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 91.09757940000236,
            "range": "± 2.605",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.4161031527912904,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 286.0721156000011,
            "range": "± 7.775",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 426.51050899999916,
            "range": "± 2.519",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6707270033526926,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 413.33580280000035,
            "range": "± 7.416",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 429.86293660000285,
            "range": "± 12.041",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 0.9615525499110863,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 217.6820895999981,
            "range": "± 2.106",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.15975499999786,
            "range": "± 3.609",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.9376392114128486,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 192.89378859999715,
            "range": "± 1.168",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 236.44175040000164,
            "range": "± 4.802",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8158194915816179,
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
          "id": "b30c2f014512b686b1f1bee95367dd7263f852e2",
          "message": "Merge pull request #476 from patbuc/465-distinct-scan-windowed\n\n465: Add distinct, scan and windowed to Array and Range",
          "timestamp": "2026-10-06T02:08:30+02:00",
          "tree_id": "41b01301fa09310b5cff00d2c62cbf866be02c97",
          "url": "https://github.com/patbuc/neon/commit/b30c2f014512b686b1f1bee95367dd7263f852e2"
        },
        "date": 1791245387861,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 243.06990279999923,
            "range": "± 1.28",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 152.0313007999988,
            "range": "± 0.858",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.5988148593148204,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 333.51164520000225,
            "range": "± 1.178",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 506.37391759999844,
            "range": "± 32.657",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6586272191520223,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 253.82330559999673,
            "range": "± 22.237",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 284.9131966000044,
            "range": "± 2.844",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8908794279415023,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 143.68371659999468,
            "range": "± 0.952",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 171.03096620000144,
            "range": "± 3.099",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8401035192186938,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 142.4498270000015,
            "range": "± 0.098",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 88.84292779999612,
            "range": "± 0.971",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6033896059873853,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 254.58806440000217,
            "range": "± 1.688",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 365.57804580000095,
            "range": "± 9.126",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6963986686970838,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 411.2771752000043,
            "range": "± 46.025",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 383.32935059999613,
            "range": "± 10.276",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.072908126018171,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 209.88556919999724,
            "range": "± 1.121",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 189.69812939999997,
            "range": "± 0.771",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1064187605004252,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 184.16652960000022,
            "range": "± 2.783",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 211.83703300000047,
            "range": "± 8.016",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8693783470806061,
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
          "id": "4f08a3e00ed0f754252fee420012e98096173a2c",
          "message": "Merge pull request #478 from patbuc/worktree-477-share-unit-variant-map-key\n\nShare the frozen map key of unit enum variants",
          "timestamp": "2026-10-06T21:15:13+02:00",
          "tree_id": "bc4c5f387ddf90d82b39e308786f299220946aae",
          "url": "https://github.com/patbuc/neon/commit/4f08a3e00ed0f754252fee420012e98096173a2c"
        },
        "date": 1791314216962,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 330.04273580000074,
            "range": "± 2.375",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 196.81769819999886,
            "range": "± 1.166",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6768956187294881,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 422.97929999999724,
            "range": "± 5.675",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.1761954000048,
            "range": "± 7.316",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7963069573956918,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 314.62571320000166,
            "range": "± 0.601",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 312.9204236000021,
            "range": "± 10.245",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0054495950771798,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 182.0484752000027,
            "range": "± 0.754",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 195.6272437999985,
            "range": "± 7.586",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9305885604876273,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.18774260000225,
            "range": "± 0.825",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.16314459999705,
            "range": "± 0.63",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6174896173035263,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 337.2795110000027,
            "range": "± 1.598",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 437.0349620000013,
            "range": "± 2.439",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7717449182017655,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 517.3546487999971,
            "range": "± 3.439",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 424.978120800003,
            "range": "± 16.604",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2173677266634322,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 252.9457064000013,
            "range": "± 1.426",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 221.71221779999826,
            "range": "± 2.347",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1408740073502766,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 237.3317371999974,
            "range": "± 1.795",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.67613920000508,
            "range": "± 6.456",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9739637946463018,
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
          "id": "4b6abf2017b181d97117309620844aece04fd0e5",
          "message": "Merge pull request #479 from patbuc/worktree-444-rename-scripts-unified-methods\n\n444: Rename scripts after the unified method names",
          "timestamp": "2026-10-06T22:54:25+02:00",
          "tree_id": "5a561cc2c053e56ffd7bcb3b238f9b9474566ead",
          "url": "https://github.com/patbuc/neon/commit/4b6abf2017b181d97117309620844aece04fd0e5"
        },
        "date": 1791320155184,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 335.2223445999982,
            "range": "± 16.175",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.47347400000172,
            "range": "± 0.745",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7976945321456024,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 421.2786531999967,
            "range": "± 1.883",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 529.4692916000002,
            "range": "± 22.052",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7956621089901874,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 314.94474959999934,
            "range": "± 0.339",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 307.33624320000104,
            "range": "± 4.294",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0247562940210961,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 182.26881599999558,
            "range": "± 0.66",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.16043059999777,
            "range": "± 3.663",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9485241859152959,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.17193620000182,
            "range": "± 0.349",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 114.26028739999765,
            "range": "± 6.798",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.559351374430426,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 337.65362899999616,
            "range": "± 1.813",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 435.7214424000034,
            "range": "± 3.704",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.774930026716522,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 516.3309926000011,
            "range": "± 3.205",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.2942604,
            "range": "± 7.565",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2708301419066788,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 255.24719619999985,
            "range": "± 1.05",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 224.37264360000313,
            "range": "± 3.008",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.137603907965883,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 237.8092807999991,
            "range": "± 2.505",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 244.99740179999776,
            "range": "± 5.852",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9706604194689924,
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
          "id": "f02c57fc5b2f90e01500068905966281d96be70d",
          "message": "Merge pull request #482 from patbuc/worktree-446-lowercase-runtime-type-names\n\n446: Name runtime types in lowercase in every error",
          "timestamp": "2026-10-06T23:18:04+02:00",
          "tree_id": "8ee661c695e88f43183ac414b2b3b61d4f7d46ab",
          "url": "https://github.com/patbuc/neon/commit/f02c57fc5b2f90e01500068905966281d96be70d"
        },
        "date": 1791321582046,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 248.10490499999673,
            "range": "± 10.849",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 156.97284359999912,
            "range": "± 3.636",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.580559409576741,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 352.3997387999998,
            "range": "± 20.138",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 516.3455749999969,
            "range": "± 35.552",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6824881549532054,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 249.32810400000562,
            "range": "± 4.934",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 293.9456094000036,
            "range": "± 11.702",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8482116964050921,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 144.41302180000548,
            "range": "± 0.837",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 173.0450871999949,
            "range": "± 4.968",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8345398539578402,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 143.89846260000922,
            "range": "± 2.66",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 92.61618440000348,
            "range": "± 2.061",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5537075245782186,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 268.1233031999966,
            "range": "± 6.615",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 409.61409619999927,
            "range": "± 28.824",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6545753812854185,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 419.9979703999986,
            "range": "± 15.671",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.4238197999998,
            "range": "± 23.917",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0333990035492473,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 249.97739379999757,
            "range": "± 13.483",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 191.39954279999642,
            "range": "± 7.154",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.3060501093318297,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 197.33587400000374,
            "range": "± 8.607",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 216.65606920000755,
            "range": "± 11.411",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9108255066597362,
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
          "id": "5d5d16318d1271ca43be45065b36f27b3b0de524",
          "message": "Merge pull request #483 from patbuc/worktree-186-parse-import-export\n\n186: Parse import and export declarations",
          "timestamp": "2026-10-06T23:27:04+02:00",
          "tree_id": "03e792cd1a19441ab7463f3dac3c518d0a5ab7aa",
          "url": "https://github.com/patbuc/neon/commit/5d5d16318d1271ca43be45065b36f27b3b0de524"
        },
        "date": 1791322121595,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 335.2380072000017,
            "range": "± 0.993",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 192.122475000005,
            "range": "± 2.66",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7449182205256981,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 417.74491319999925,
            "range": "± 1.544",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.5023153999988,
            "range": "± 5.234",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.785970072182305,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 314.3317536000012,
            "range": "± 0.521",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.9411607999988,
            "range": "± 7.25",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0109042906744126,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 181.89539019999756,
            "range": "± 0.366",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 193.08774999999798,
            "range": "± 6.539",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9420348530655076,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.04000580000547,
            "range": "± 0.159",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.47041660000048,
            "range": "± 0.614",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6116532487124222,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 336.71124759999884,
            "range": "± 2.752",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 441.03535539999825,
            "range": "± 7.202",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7634563612130769,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 515.1592550000032,
            "range": "± 1.719",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 406.0083001999999,
            "range": "± 9.406",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2688392201495278,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 255.33311219999746,
            "range": "± 4.575",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 230.1341760000014,
            "range": "± 4.922",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1094967146470063,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 241.87076860000332,
            "range": "± 4.282",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 236.81760380000014,
            "range": "± 4.079",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0213377921189961,
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
          "id": "12ff32f53448c0e80b888ec37bbdd7cf61883def",
          "message": "Merge pull request #485 from patbuc/worktree-187-resolve-module-graph\n\n187: Resolve the module graph at compile time",
          "timestamp": "2026-10-07T00:04:08+02:00",
          "tree_id": "2217650494e1d3d806f795ec96273fe61bc4fdde",
          "url": "https://github.com/patbuc/neon/commit/12ff32f53448c0e80b888ec37bbdd7cf61883def"
        },
        "date": 1791324352232,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 334.1983580000033,
            "range": "± 0.417",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.62308780000205,
            "range": "± 0.881",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7717786401331495,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 416.4850350000023,
            "range": "± 0.887",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 526.5215681999962,
            "range": "± 9.662",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7910122968444151,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 314.7817837999952,
            "range": "± 2.395",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 318.1383179999983,
            "range": "± 11.748",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9894494500973531,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 183.26793380000197,
            "range": "± 1.055",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 193.5245493999986,
            "range": "± 3.744",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9470009586287832,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.4672214000011,
            "range": "± 0.429",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.73901179999746,
            "range": "± 2.196",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5971791635265309,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 340.6476871999985,
            "range": "± 3.616",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.90531199999634,
            "range": "± 1.979",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7691207984428087,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 521.7954309999953,
            "range": "± 2.598",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 414.2112475999994,
            "range": "± 17.199",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2597326461397522,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 262.766461999999,
            "range": "± 2.449",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 233.29978919999803,
            "range": "± 2.983",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1263038980919973,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 234.66870160000042,
            "range": "± 1.683",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 240.92262840000274,
            "range": "± 4.107",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9740417625295912,
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
          "id": "77e1b96a60755c3c36d8b1e6316989b2dfe265b2",
          "message": "Merge pull request #486 from patbuc/worktree-188-compile-modules-prep\n\n188: Prepare compiling modules as units",
          "timestamp": "2026-10-07T00:17:02+02:00",
          "tree_id": "0b6326a703e1157abaad07b143de8c964a9ca71d",
          "url": "https://github.com/patbuc/neon/commit/77e1b96a60755c3c36d8b1e6316989b2dfe265b2"
        },
        "date": 1791325117031,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 340.89436259999957,
            "range": "± 2.218",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.2551465999976,
            "range": "± 0.507",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8204806051509825,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 421.63878799999566,
            "range": "± 6.378",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 533.2435356000019,
            "range": "± 13.85",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7907058592385384,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 318.5942343999983,
            "range": "± 5.928",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.78405220000036,
            "range": "± 4.67",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0251305758603464,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 185.21484919999693,
            "range": "± 4.618",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 196.38960639999823,
            "range": "± 8.337",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9430990396852214,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.1941032000011,
            "range": "± 3.201",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.38250379999965,
            "range": "± 0.629",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6233922680779205,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 350.34559479999814,
            "range": "± 6.672",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 436.9636287999981,
            "range": "± 4.461",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8017728975798905,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 517.8822064000002,
            "range": "± 1.735",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 407.9918703999965,
            "range": "± 7.418",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2693444256432533,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 257.73690480000084,
            "range": "± 2.632",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 219.27088479999952,
            "range": "± 4.591",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1754269383967086,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 235.23789239999644,
            "range": "± 0.89",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 239.5865963999995,
            "range": "± 3.861",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9818491348625249,
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
          "id": "417d22c9e5b7c0809754cd7a5f73ba5a243366c1",
          "message": "Merge pull request #488 from patbuc/worktree-188-compile-modules\n\n188: Compile modules with exports and namespace bindings",
          "timestamp": "2026-10-07T01:05:20+02:00",
          "tree_id": "206e92b001c79f478ad78a6faa4ccb70c2694985",
          "url": "https://github.com/patbuc/neon/commit/417d22c9e5b7c0809754cd7a5f73ba5a243366c1"
        },
        "date": 1791328024588,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 382.0662027999987,
            "range": "± 2.176",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.37299180000184,
            "range": "± 0.616",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0500084218747565,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 429.8233390000007,
            "range": "± 3.154",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 520.8912077999969,
            "range": "± 15.22",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8251691189324837,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 340.4992460000045,
            "range": "± 3.048",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 315.9460442000011,
            "range": "± 10.489",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0777132749427958,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 191.67016959999899,
            "range": "± 5.687",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.9078189999974,
            "range": "± 4.771",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0039932916524577,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 184.04798519999588,
            "range": "± 1.243",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.93665940000312,
            "range": "± 1.239",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6741275040051895,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 371.48841559999823,
            "range": "± 2.925",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 430.69903940000245,
            "range": "± 1.68",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8625243653143753,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 528.2426270000002,
            "range": "± 2.603",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 420.6546478000064,
            "range": "± 22.671",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2557632009123665,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 255.35442839999973,
            "range": "± 0.469",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 214.7737042000017,
            "range": "± 5.064",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.188946427828094,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 244.0939947999965,
            "range": "± 3.056",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 239.8621095999971,
            "range": "± 7.408",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.017642991663238,
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
          "id": "6f596d437773d728de141741e9692d05d0dc9954",
          "message": "Merge pull request #489 from patbuc/worktree-189-run-modules\n\n189: Run modules in dependency order",
          "timestamp": "2026-10-07T01:36:25+02:00",
          "tree_id": "5e5b606501eaf187812bbefe8841b8d5521adb88",
          "url": "https://github.com/patbuc/neon/commit/6f596d437773d728de141741e9692d05d0dc9954"
        },
        "date": 1791329848495,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 203.7892364000001,
            "range": "± 4.489",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 100.722814800001,
            "range": "± 0.698",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 2.0232678842886953,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 230.40465740000116,
            "range": "± 0.753",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 281.1073783999973,
            "range": "± 13.119",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8196321943287824,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 188.40675420000252,
            "range": "± 15.256",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 173.88385660000267,
            "range": "± 5.117",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0835206780202022,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 105.04113880000148,
            "range": "± 0.87",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 105.89230539999903,
            "range": "± 1.814",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9919619598725106,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 101.35507379999353,
            "range": "± 1.284",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 58.77190380000172,
            "range": "± 2.848",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7245497805363004,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 199.6655043999965,
            "range": "± 1.371",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 251.581145199998,
            "range": "± 7.43",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7936425610960217,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 280.6387144000041,
            "range": "± 1.789",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 233.73987619999923,
            "range": "± 10.063",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2006454309913126,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 140.86078900000416,
            "range": "± 3.9",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 129.43465760000095,
            "range": "± 2.589",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0882772173378326,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 131.80957019999937,
            "range": "± 1.335",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 131.54788960000303,
            "range": "± 5.726",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0019892420987675,
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
          "id": "36a85d305a55e1218e7896691604a09c780d4075",
          "message": "Merge pull request #490 from patbuc/worktree-190-multi-file-script-tests\n\n190: Run multi-file script tests",
          "timestamp": "2026-10-07T07:49:29+02:00",
          "tree_id": "7056ba09e40b4470c91b92d07713b453ea6572c6",
          "url": "https://github.com/patbuc/neon/commit/36a85d305a55e1218e7896691604a09c780d4075"
        },
        "date": 1791352241602,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 199.7429244000017,
            "range": "± 4.922",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 108.61054420000471,
            "range": "± 1.57",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.8390748879057088,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 237.67967259999807,
            "range": "± 1.211",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 283.59623119999924,
            "range": "± 10.03",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8380917884355816,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 179.68462880000118,
            "range": "± 2.388",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 177.80627120000077,
            "range": "± 4.392",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0105640683386672,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 110.38108799999691,
            "range": "± 8.922",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 113.01521499999865,
            "range": "± 0.844",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9766922798845998,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 102.74238900000796,
            "range": "± 0.893",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 58.74122740000303,
            "range": "± 0.615",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7490677935681451,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 199.20463340000083,
            "range": "± 1.977",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 265.0851380000006,
            "range": "± 5.913",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7514741675182122,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 296.02731439999843,
            "range": "± 3.862",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 248.2266119999963,
            "range": "± 20.864",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1925688064420863,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 163.18971860000033,
            "range": "± 1.419",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 142.33091580000234,
            "range": "± 3.535",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1465514549861249,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 139.99384540000506,
            "range": "± 1.89",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 143.0102269999992,
            "range": "± 10.426",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9789079308293515,
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
          "id": "5f33326c6e028a1d587dbf633e887a3061467959",
          "message": "Merge pull request #491 from patbuc/worktree-191-impl-methods-on-struct-prep\n\n191: Key struct methods by declaration",
          "timestamp": "2026-10-07T08:02:52+02:00",
          "tree_id": "b484af4028184269f690e4db7982bbf2043f730f",
          "url": "https://github.com/patbuc/neon/commit/5f33326c6e028a1d587dbf633e887a3061467959"
        },
        "date": 1791353033759,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 195.24654479998844,
            "range": "± 8.222",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 101.13530500000252,
            "range": "± 2.632",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.9305478418242135,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 226.6271818000007,
            "range": "± 2.82",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 272.74313980000784,
            "range": "± 14.087",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8309179910672649,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 177.1675115999983,
            "range": "± 1.202",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 175.57396000000267,
            "range": "± 11.613",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0090762411464411,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 103.90985059999593,
            "range": "± 0.747",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 109.16108140000915,
            "range": "± 3.467",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9518946612413023,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 100.55517799999052,
            "range": "± 0.373",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 60.04289779999681,
            "range": "± 1.091",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6747222683179004,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 198.63762319999978,
            "range": "± 1.322",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 258.72580400000516,
            "range": "± 7.548",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7677534290317475,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 289.6019122000041,
            "range": "± 10.93",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 234.66920779999896,
            "range": "± 4.321",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2340856941351357,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 154.191486000002,
            "range": "± 3.602",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 141.78452220000395,
            "range": "± 21.659",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0875057700762043,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 132.56466440000167,
            "range": "± 1.58",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 128.591586400006,
            "range": "± 3.532",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.030896873669765,
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
          "id": "63674cdebafe5346fdf3fe3b9dc48657e4a68f2f",
          "message": "Merge pull request #493 from patbuc/worktree-191-impl-methods-on-struct\n\n191: Attach impl methods to the struct they implement",
          "timestamp": "2026-10-07T08:43:15+02:00",
          "tree_id": "434e8e95a9744ccaffede7e3a4a69be2af189853",
          "url": "https://github.com/patbuc/neon/commit/63674cdebafe5346fdf3fe3b9dc48657e4a68f2f"
        },
        "date": 1791355471024,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 241.8259489999997,
            "range": "± 0.922",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 153.79732920000038,
            "range": "± 3.117",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.572367675419939,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 332.7184656000014,
            "range": "± 0.752",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 487.0933507999979,
            "range": "± 31.305",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.6830692002950718,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 250.07512200000122,
            "range": "± 11.186",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 282.0770334000031,
            "range": "± 12.675",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8865490358634723,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 144.15916539999785,
            "range": "± 1.093",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 173.48935360000155,
            "range": "± 7.489",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8309395499413317,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 142.47793439999725,
            "range": "± 0.809",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.71076599999503,
            "range": "± 1.625",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6244064542772927,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 252.61535140000433,
            "range": "± 0.309",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 361.7513092000024,
            "range": "± 11.61",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6983121967371795,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 390.98625119999895,
            "range": "± 1.691",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 382.8442231999986,
            "range": "± 10.301",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0212672087146706,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 203.75307999999848,
            "range": "± 0.28",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 186.11450360000106,
            "range": "± 6.245",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.094772712812895,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 187.27915199999927,
            "range": "± 4.482",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 215.44612900000288,
            "range": "± 8.707",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8692620882503614,
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
          "id": "17f4e83cb77a1c03af4b9bc37068c430417927d9",
          "message": "Merge pull request #494 from patbuc/worktree-192-std-modules\n\n192: Move the standard library into std modules",
          "timestamp": "2026-10-07T09:53:41+02:00",
          "tree_id": "56e6260c535f7c8c56baeb2950f1bf8e595610c2",
          "url": "https://github.com/patbuc/neon/commit/17f4e83cb77a1c03af4b9bc37068c430417927d9"
        },
        "date": 1791359712848,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 241.91799400000207,
            "range": "± 0.874",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 152.77539739999781,
            "range": "± 0.429",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.58348790523261,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 332.99950800000033,
            "range": "± 0.358",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 494.41344219999905,
            "range": "± 16.225",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.673524381777015,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 245.89479640000036,
            "range": "± 1.61",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 283.4009358000003,
            "range": "± 5.898",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8676569669958023,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 142.11363639999774,
            "range": "± 0.283",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 168.397303200004,
            "range": "± 3.818",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8439187190023503,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 142.89522099999772,
            "range": "± 0.343",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 89.16185859999644,
            "range": "± 6.035",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6026496446318295,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 253.29101079999816,
            "range": "± 1.51",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 369.4832577999989,
            "range": "± 4.304",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6855277078267629,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 390.6213146000056,
            "range": "± 0.841",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 387.2437194000014,
            "range": "± 13.982",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.008722143267391,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 201.60326160000182,
            "range": "± 0.396",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 183.24471539999365,
            "range": "± 5.429",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1001859516654242,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 196.48950200000286,
            "range": "± 27.244",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 214.96535500000107,
            "range": "± 9.942",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9140519503712674,
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
          "id": "567fc5f087e9db5ae332d7f635ebc90bad27eb57",
          "message": "Merge pull request #495 from patbuc/worktree-328-predecode-instr\n\n328: Pre-decode bytecode into a typed instruction stream",
          "timestamp": "2026-10-07T13:34:53+02:00",
          "tree_id": "5efea52967a1009b2dbb2600928b5a03490331f0",
          "url": "https://github.com/patbuc/neon/commit/567fc5f087e9db5ae332d7f635ebc90bad27eb57"
        },
        "date": 1791372991765,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 347.794847199998,
            "range": "± 2.045",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 190.33985399999835,
            "range": "± 4.66",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.827230818407589,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 424.7834102000013,
            "range": "± 0.717",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 527.8647706000015,
            "range": "± 11.407",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8047201364038141,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 312.17454579999924,
            "range": "± 1.205",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 310.2352501999974,
            "range": "± 12.066",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 1.0062510485148017,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 180.10937439999566,
            "range": "± 0.834",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 190.13256259999594,
            "range": "± 1.462",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.9472831583241886,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 179.63094840000053,
            "range": "± 1.658",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 109.56281260000083,
            "range": "± 0.37",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6395247998589548,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 359.77297099999817,
            "range": "± 1.983",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 434.9311024000059,
            "range": "± 6.343",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.8271953167173479,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 515.0735371999986,
            "range": "± 1.396",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 417.2034155999995,
            "range": "± 12.415",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.2345860986282855,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 245.8191274000029,
            "range": "± 1.766",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 213.22774299999878,
            "range": "± 3.331",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1528477670938173,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 237.13259919999814,
            "range": "± 0.622",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 242.02542159999894,
            "range": "± 4.159",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9797838492846951,
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
          "id": "25ca36b46e1814e99105d911f17781973a0da185",
          "message": "Merge pull request #496 from patbuc/worktree-423-fused-condition-jumps-prep\n\n423: Extract the comparison and number-literal helpers",
          "timestamp": "2026-10-07T15:52:47+02:00",
          "tree_id": "d82c8f794e922e50a014bf942d79975006da7d08",
          "url": "https://github.com/patbuc/neon/commit/25ca36b46e1814e99105d911f17781973a0da185"
        },
        "date": 1791381233817,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 193.7245455999971,
            "range": "± 4.498",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 104.29658559999666,
            "range": "± 1.694",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.85743899942208,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 251.78373060000467,
            "range": "± 4.788",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 276.48156899999776,
            "range": "± 5.951",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.9106709409624578,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 176.67475160000095,
            "range": "± 1.226",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 180.0064779999957,
            "range": "± 16.418",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9814910750045627,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 111.10512620000463,
            "range": "± 4.844",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 110.98506920000091,
            "range": "± 2.962",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 1.0010817401013408,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 101.65934220001418,
            "range": "± 0.32",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 58.445147999998426,
            "range": "± 1.154",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7393974637555356,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 196.96434560000853,
            "range": "± 0.805",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 258.9928802000031,
            "range": "± 4.887",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7605010046913488,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 278.6136742000167,
            "range": "± 4.545",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 237.56845359999943,
            "range": "± 6.647",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1727721840927847,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 156.6075375999958,
            "range": "± 1.722",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 144.7768291999978,
            "range": "± 2.976",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.081716863571137,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 140.59678200000008,
            "range": "± 1.19",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 132.2090289999892,
            "range": "± 3.664",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 1.0634431177919896,
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
          "id": "d99f7f5571105f4d4ff9fa83a18014ae5ae6f544",
          "message": "Merge pull request #497 from patbuc/423-fused-condition-jumps\n\n422/423: Pop and fuse conditions into the conditional jump",
          "timestamp": "2026-10-07T16:07:02+02:00",
          "tree_id": "85304d4b4e70b55a15d32dc4e94b1c6839220b9a",
          "url": "https://github.com/patbuc/neon/commit/d99f7f5571105f4d4ff9fa83a18014ae5ae6f544"
        },
        "date": 1791382095689,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 186.02556779999873,
            "range": "± 9.411",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 109.1491359999992,
            "range": "± 3.612",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.7043246938757268,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 240.77370099999484,
            "range": "± 4.877",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 280.72235200000364,
            "range": "± 4.655",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.8576933731304436,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 167.25284400000078,
            "range": "± 0.821",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 177.04066280000177,
            "range": "± 5.897",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9447142896710795,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 97.64372340000591,
            "range": "± 1.049",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 109.25459499999874,
            "range": "± 1.981",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8937264688959492,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 98.94342880000409,
            "range": "± 0.566",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 59.176827399994636,
            "range": "± 1.641",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6719961705823563,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 188.56794920000084,
            "range": "± 2.101",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 255.3785105999964,
            "range": "± 6.382",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.738386126369724,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 263.5225042000002,
            "range": "± 3.484",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 245.85183600000278,
            "range": "± 15.673",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0718752745047517,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 149.97190639999758,
            "range": "± 1.125",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 136.34321259999354,
            "range": "± 4.209",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0999587257782182,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 135.64190579999433,
            "range": "± 1.654",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 139.50245079999775,
            "range": "± 11.168",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9723263284776393,
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
          "id": "229d0cd9cd762d47e07c44638783d9e2d86e381c",
          "message": "Merge pull request #498 from patbuc/version-from-merge-count\n\nDerive the patch version from the merge count",
          "timestamp": "2026-10-07T17:43:37+02:00",
          "tree_id": "c098b5c3a621ecef9afb599f6e6073daa96025b2",
          "url": "https://github.com/patbuc/neon/commit/229d0cd9cd762d47e07c44638783d9e2d86e381c"
        },
        "date": 1791387929560,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 303.36146579999195,
            "range": "± 1.556",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.59677800000918,
            "range": "± 3.732",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6000349214794,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 393.5324401999992,
            "range": "± 8.511",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 538.7166004000051,
            "range": "± 2.981",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7304999324464766,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 305.2947786000004,
            "range": "± 0.759",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 312.33132480001586,
            "range": "± 6.402",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9774708918341095,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 170.0507995999942,
            "range": "± 1.942",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 195.43085099998052,
            "range": "± 11.66",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8701328307679076,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 175.37100620000956,
            "range": "± 0.92",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 110.49769180001476,
            "range": "± 0.622",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5871010818706182,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 339.291630799994,
            "range": "± 2.381",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 446.1081290000038,
            "range": "± 3.012",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7605591755535821,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 456.98493160000453,
            "range": "± 0.965",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 408.7234935999845,
            "range": "± 10.336",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.118078453418323,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 266.6606510000065,
            "range": "± 9.314",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 236.12824959998306,
            "range": "± 3.356",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.1293043142942336,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 228.50721920000296,
            "range": "± 1.431",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 243.7519478000013,
            "range": "± 6.899",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9374580234636457,
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
          "id": "accee538bf78cda351ced032c148c20db51e5f9c",
          "message": "Merge pull request #501 from patbuc/385-pin-compile-errors\n\n385: Pin compile errors by line, column, code and message",
          "timestamp": "2026-10-07T17:44:23+02:00",
          "tree_id": "71145d162005ff23f3540215280975d8a803411b",
          "url": "https://github.com/patbuc/neon/commit/accee538bf78cda351ced032c148c20db51e5f9c"
        },
        "date": 1791388037215,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 308.40084339999976,
            "range": "± 8.571",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.20791560000498,
            "range": "± 0.834",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.656217687665215,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 401.7110004000017,
            "range": "± 0.58",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 531.5875248000054,
            "range": "± 8.963",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.7556817676470755,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 293.8280470000052,
            "range": "± 1.668",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 312.4846015999992,
            "range": "± 14.209",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9402960833766918,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 171.6512734000105,
            "range": "± 0.685",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 196.15792740000302,
            "range": "± 7.193",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8750667162687805,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 180.2305132000015,
            "range": "± 1.189",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.12140899999758,
            "range": "± 0.374",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6219242972342567,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 340.89795879999656,
            "range": "± 1.275",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 441.28613740000446,
            "range": "± 3.273",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7725100108707678,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 457.850743400013,
            "range": "± 2.615",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 413.9055773999985,
            "range": "± 5.639",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.1061719590155363,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 251.11979540001244,
            "range": "± 5.179",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 235.14050100000077,
            "range": "± 5.109",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0679563679249438,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 228.56978059999733,
            "range": "± 3.868",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 251.42400080000584,
            "range": "± 11.864",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9091008808733905,
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
          "id": "9ce4facadf48a72dd12217675febad49e79381f9",
          "message": "Merge pull request #499 from patbuc/worktree-424-modulo-multiply-constant\n\n424: Add ModuloConstant and MultiplyConstant",
          "timestamp": "2026-10-07T18:07:03+02:00",
          "tree_id": "d644103a93c09dbd088ca45cbe37620c50f0ffb7",
          "url": "https://github.com/patbuc/neon/commit/9ce4facadf48a72dd12217675febad49e79381f9"
        },
        "date": 1791389306729,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 202.05040360000055,
            "range": "± 1.612",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 121.70552620000308,
            "range": "± 0.323",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6601580052163272,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 207.21782860000246,
            "range": "± 0.183",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 388.529797999999,
            "range": "± 4.419",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.5333383170780713,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 187.8416417999972,
            "range": "± 2.802",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 217.5613730000009,
            "range": "± 5.462",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8633961038662705,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 108.307591800002,
            "range": "± 1.178",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 135.14305600000114,
            "range": "± 2.551",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8014292040280715,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 115.36166019999712,
            "range": "± 0.081",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 67.4324745999968,
            "range": "± 0.605",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.710773049399593,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 239.79617719999737,
            "range": "± 1.154",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 340.04605480000123,
            "range": "± 17.372",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.705187352757361,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 328.8770492000083,
            "range": "± 2.852",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 301.9272933999986,
            "range": "± 14.324",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0892590911425362,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 163.0276121999998,
            "range": "± 0.869",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 176.7949912000006,
            "range": "± 3.83",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 0.9221280031376774,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 155.86457619999692,
            "range": "± 2.439",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 181.045283200001,
            "range": "± 5.697",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8609148686177761,
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
          "id": "9acacd65efa69c5ffde0c09d4bb19251c90f54a7",
          "message": "Merge pull request #515 from patbuc/worktree-508-version-flag\n\n508: Add --version and -V",
          "timestamp": "2026-10-07T18:24:59+02:00",
          "tree_id": "1f74b3abe830b9b9b4e6471a0ab53c7940cbcdcd",
          "url": "https://github.com/patbuc/neon/commit/9acacd65efa69c5ffde0c09d4bb19251c90f54a7"
        },
        "date": 1791390407144,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 308.55419159999826,
            "range": "± 3.146",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 187.70715540000538,
            "range": "± 1.818",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6438062307346086,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 313.8365890000017,
            "range": "± 0.979",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 535.6756834000038,
            "range": "± 6.822",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.5858705159212002,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 307.140841399999,
            "range": "± 79.334",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 320.08447700000033,
            "range": "± 25.551",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.959561814051978,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 161.6464282000038,
            "range": "± 0.372",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 199.15253799999846,
            "range": "± 6.718",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8116714445286409,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 180.41103859999907,
            "range": "± 1.052",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 112.88356199999328,
            "range": "± 2.318",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5982046934345482,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 337.6863722000053,
            "range": "± 1.534",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 441.58640839999634,
            "range": "± 3.264",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7647118792073947,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 455.8165966000047,
            "range": "± 1.236",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 415.90060180000137,
            "range": "± 21.796",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.095974842612029,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 237.37580380000622,
            "range": "± 5.09",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 232.65664880000259,
            "range": "± 6.446",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0202837744992208,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 215.6816794000008,
            "range": "± 1.624",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 244.08025980000048,
            "range": "± 8.882",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8836506466222648,
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
          "id": "85312f17644f55e4f5305e1e832dc446f1025d4b",
          "message": "Merge pull request #516 from patbuc/worktree-500-rename-module-keywords\n\n500: Rename module keywords to use and pub",
          "timestamp": "2026-10-07T18:29:42+02:00",
          "tree_id": "861cfb261d5e57abf67694a92095f9f2be0a6c09",
          "url": "https://github.com/patbuc/neon/commit/85312f17644f55e4f5305e1e832dc446f1025d4b"
        },
        "date": 1791390695992,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 302.67754620000176,
            "range": "± 2.002",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 189.23713119999945,
            "range": "± 1.406",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.5994617138858984,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 304.194627199999,
            "range": "± 1.93",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 526.6925470000018,
            "range": "± 11.103",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.5775563541437353,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 289.11651439999844,
            "range": "± 33.99",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 308.32843340000124,
            "range": "± 8.866",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.9376900833045172,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 157.11630960000207,
            "range": "± 0.484",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 201.3850596000026,
            "range": "± 7.391",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.7801785788482596,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.5737358000015,
            "range": "± 0.324",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.57291819999955,
            "range": "± 1.973",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5915487258448549,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 340.99067440000397,
            "range": "± 1.44",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 458.97503119999783,
            "range": "± 13.184",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7429394873801265,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 456.44649680000384,
            "range": "± 1.21",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 418.768559199998,
            "range": "± 22.7",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0899731767637584,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 251.27910799999995,
            "range": "± 2.764",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 244.23940580000476,
            "range": "± 8.29",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0288229582648085,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 219.7488614000008,
            "range": "± 3.8",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 245.04795420000391,
            "range": "± 13.626",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8967586043205469,
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
          "id": "b6aaa57b49e518dc596337804f385a236d86bdca",
          "message": "Merge pull request #517 from patbuc/worktree-509-eval-flag\n\n509: Add -e and --eval to run a snippet",
          "timestamp": "2026-10-07T18:34:31+02:00",
          "tree_id": "51cd147e4de5fca5bc7eb20e23d134527b6b6ab5",
          "url": "https://github.com/patbuc/neon/commit/b6aaa57b49e518dc596337804f385a236d86bdca"
        },
        "date": 1791390960712,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 224.54693139999904,
            "range": "± 1.696",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 151.7915252000023,
            "range": "± 0.588",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.4793113851654982,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 238.93994220000252,
            "range": "± 0.303",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 502.15858739999817,
            "range": "± 16.978",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.47582566184350267,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 220.71722139999963,
            "range": "± 0.341",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 275.3781556000007,
            "range": "± 13.425",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8015059179951857,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 124.92286279999689,
            "range": "± 0.602",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 175.04732020000233,
            "range": "± 10.813",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.7136519579806468,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 139.47359839999933,
            "range": "± 0.963",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.58019519999891,
            "range": "± 1.184",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5925244067051483,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 254.92593939999608,
            "range": "± 0.674",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 361.7005880000022,
            "range": "± 6.225",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7047982443423468,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 358.57602320000126,
            "range": "± 3.419",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 375.56634180000117,
            "range": "± 24.529",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 0.9547608059908422,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 190.31461619999845,
            "range": "± 1.142",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 186.59321860000375,
            "range": "± 6.314",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0199439059356825,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 176.13833959999852,
            "range": "± 5.301",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 214.37733320000234,
            "range": "± 1.937",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8216276271879502,
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
          "id": "40febaae20a25f57601db5253ffd914c0f4c4b53",
          "message": "Merge pull request #519 from patbuc/worktree-510-run-stdin-prep\n\n510: Generalise run_eval to run_source",
          "timestamp": "2026-10-07T18:44:24+02:00",
          "tree_id": "9a12e997ec4d158a7549a9382bd415fde3034b0f",
          "url": "https://github.com/patbuc/neon/commit/40febaae20a25f57601db5253ffd914c0f4c4b53"
        },
        "date": 1791391549697,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 225.15529439999398,
            "range": "± 4.281",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 160.33373959999722,
            "range": "± 3.665",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.4042914171509655,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 238.79299040000603,
            "range": "± 0.542",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 497.19467800000245,
            "range": "± 5.636",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.4802806646293283,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 220.17668000000015,
            "range": "± 0.431",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 288.32740559999763,
            "range": "± 16.585",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.7636342426132591,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 125.65318679999962,
            "range": "± 0.359",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 172.01129379999998,
            "range": "± 2.21",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.730493818307642,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 140.49415899999929,
            "range": "± 4.146",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 87.11980559999688,
            "range": "± 0.86",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6126546430218884,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 257.8762088000019,
            "range": "± 5.884",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 368.5294470000031,
            "range": "± 14.269",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.6997438356669494,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 357.2299265999959,
            "range": "± 1.656",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 386.3909064000012,
            "range": "± 9.127",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 0.924529849649678,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 196.31535679999956,
            "range": "± 4.173",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 190.2874338000032,
            "range": "± 5.012",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0316779877662958,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 174.3313102000002,
            "range": "± 0.57",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 220.17356480000103,
            "range": "± 7.41",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.7917903784605452,
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
          "id": "008e458df9b7949e74f07381b172fea29f4aa03a",
          "message": "Merge pull request #518 from patbuc/worktree-510-run-stdin\n\n510: Run a script from stdin with neon -",
          "timestamp": "2026-10-07T18:48:49+02:00",
          "tree_id": "f53f31f926093e888d0a95c489d11bbd27a23c89",
          "url": "https://github.com/patbuc/neon/commit/008e458df9b7949e74f07381b172fea29f4aa03a"
        },
        "date": 1791391821019,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 310.8577719999971,
            "range": "± 3.076",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 186.28683720000083,
            "range": "± 0.84",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6687049749320437,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 315.0035902000013,
            "range": "± 0.167",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 546.2774961999997,
            "range": "± 21.361",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.5766365856020438,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 272.9597595999991,
            "range": "± 1.512",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 318.941143400005,
            "range": "± 20.309",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8558311313810723,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 161.96036060000267,
            "range": "± 2.047",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 192.35635039999863,
            "range": "± 4.374",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8419808353777325,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 177.21892819999425,
            "range": "± 0.435",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.82576740000059,
            "range": "± 0.435",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.5847772147727137,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 340.44505780000236,
            "range": "± 2.019",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 442.84041999999886,
            "range": "± 2.023",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7687759346809472,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 455.3451525999975,
            "range": "± 1.146",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 419.37052739999956,
            "range": "± 14.945",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.0857824354587633,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 249.4653048000032,
            "range": "± 5.223",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 246.17741440000032,
            "range": "± 4.69",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.013355775987884,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 217.61221420000538,
            "range": "± 4.365",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 239.7962782000036,
            "range": "± 2.623",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.9074878719281229,
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
          "id": "c5414830cd9839a83d5d4873a785938092bf15c2",
          "message": "Merge pull request #514 from patbuc/worktree-506-available-methods-help\n\n506: Move the available-methods list into a help line",
          "timestamp": "2026-10-07T19:56:27+02:00",
          "tree_id": "a44ba5057a1fccfe62e33ee393bcbd12497a9db7",
          "url": "https://github.com/patbuc/neon/commit/c5414830cd9839a83d5d4873a785938092bf15c2"
        },
        "date": 1791395873610,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 173.8949754000089,
            "range": "± 1",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 108.4473963999983,
            "range": "± 1.52",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.6034960835630685,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 181.74709200000052,
            "range": "± 1.068",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 308.97041339999873,
            "range": "± 3.212",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.588234614440922,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 162.56016180000188,
            "range": "± 0.841",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 189.43486580000695,
            "range": "± 6.998",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8581322192907316,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 95.9292142000038,
            "range": "± 0.414",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 115.42695780000258,
            "range": "± 1.599",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.8310815430674172,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 100.05597220000482,
            "range": "± 0.085",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 56.698007600004985,
            "range": "± 0.284",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.7647176053501397,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 211.62140559999898,
            "range": "± 4.605",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 282.79978659999756,
            "range": "± 15.902",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.748308222379687,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 326.5657585999975,
            "range": "± 2.996",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 291.7446429999984,
            "range": "± 13.118",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.119354772865527,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 153.35613739999872,
            "range": "± 8.792",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 145.1939234000065,
            "range": "± 2.539",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.056215947670933,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 143.42172700000333,
            "range": "± 18.915",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 166.76539059999413,
            "range": "± 4.097",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8600209341038678,
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
          "id": "c5414830cd9839a83d5d4873a785938092bf15c2",
          "message": "Merge pull request #514 from patbuc/worktree-506-available-methods-help\n\n506: Move the available-methods list into a help line",
          "timestamp": "2026-10-07T19:56:27+02:00",
          "tree_id": "a44ba5057a1fccfe62e33ee393bcbd12497a9db7",
          "url": "https://github.com/patbuc/neon/commit/c5414830cd9839a83d5d4873a785938092bf15c2"
        },
        "date": 1791395968484,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "fib neon (ms)",
            "value": 304.16765059999875,
            "range": "± 1.092",
            "unit": "ms"
          },
          {
            "name": "fib python (ms)",
            "value": 188.26645660000167,
            "range": "± 1.616",
            "unit": "ms"
          },
          {
            "name": "fib neon/python",
            "value": 1.615623176284904,
            "unit": "ratio"
          },
          {
            "name": "loop_arith neon (ms)",
            "value": 305.0566625999977,
            "range": "± 0.727",
            "unit": "ms"
          },
          {
            "name": "loop_arith python (ms)",
            "value": 530.2137942000059,
            "range": "± 11.013",
            "unit": "ms"
          },
          {
            "name": "loop_arith neon/python",
            "value": 0.5753465223594787,
            "unit": "ratio"
          },
          {
            "name": "closures neon (ms)",
            "value": 273.0307461999985,
            "range": "± 0.855",
            "unit": "ms"
          },
          {
            "name": "closures python (ms)",
            "value": 304.3337727999983,
            "range": "± 2.842",
            "unit": "ms"
          },
          {
            "name": "closures neon/python",
            "value": 0.8971424488580453,
            "unit": "ratio"
          },
          {
            "name": "structs neon (ms)",
            "value": 158.4350537999967,
            "range": "± 1.158",
            "unit": "ms"
          },
          {
            "name": "structs python (ms)",
            "value": 198.2377452000037,
            "range": "± 13.945",
            "unit": "ms"
          },
          {
            "name": "structs neon/python",
            "value": 0.799217392430237,
            "unit": "ratio"
          },
          {
            "name": "nbody neon (ms)",
            "value": 178.68467659999396,
            "range": "± 0.887",
            "unit": "ms"
          },
          {
            "name": "nbody python (ms)",
            "value": 111.55819759999872,
            "range": "± 1.983",
            "unit": "ms"
          },
          {
            "name": "nbody neon/python",
            "value": 1.6017171345908874,
            "unit": "ratio"
          },
          {
            "name": "binary_trees neon (ms)",
            "value": 341.1893519999978,
            "range": "± 3.375",
            "unit": "ms"
          },
          {
            "name": "binary_trees python (ms)",
            "value": 449.00264180000136,
            "range": "± 2.675",
            "unit": "ms"
          },
          {
            "name": "binary_trees neon/python",
            "value": 0.7598827272646056,
            "unit": "ratio"
          },
          {
            "name": "sieve neon (ms)",
            "value": 455.6740091999984,
            "range": "± 0.966",
            "unit": "ms"
          },
          {
            "name": "sieve python (ms)",
            "value": 419.72571200000175,
            "range": "± 28.429",
            "unit": "ms"
          },
          {
            "name": "sieve neon/python",
            "value": 1.085647117086781,
            "unit": "ratio"
          },
          {
            "name": "collections neon (ms)",
            "value": 243.67260740000916,
            "range": "± 1.705",
            "unit": "ms"
          },
          {
            "name": "collections python (ms)",
            "value": 230.78593579999165,
            "range": "± 4.879",
            "unit": "ms"
          },
          {
            "name": "collections neon/python",
            "value": 1.0558382015582857,
            "unit": "ratio"
          },
          {
            "name": "strings neon (ms)",
            "value": 219.162925400002,
            "range": "± 0.844",
            "unit": "ms"
          },
          {
            "name": "strings python (ms)",
            "value": 253.00127459999544,
            "range": "± 21.815",
            "unit": "ms"
          },
          {
            "name": "strings neon/python",
            "value": 0.8662522580034699,
            "unit": "ratio"
          }
        ]
      }
    ]
  }
}