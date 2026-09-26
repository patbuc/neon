window.BENCHMARK_DATA = {
  "lastUpdate": 1790457664362,
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
      }
    ]
  }
}