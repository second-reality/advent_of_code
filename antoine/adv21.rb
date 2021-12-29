#!/usr/bin/env ruby

require 'set'

# ##############################################################################

# load "adv21.rb" ; d21011()

# ##############################################################################

# ###########################################################################
#
# 2021 DAY 23
#
# ###########################################################################

# input            example
# #############    #############
# #...........#    #...........#
# ###B#A#B#C###    ###B#C#B#D###
#   #D#A#D#C#        #A#D#C#A#
#   #########        #########

module D2123
  HALLWAY = [[1, 1], [2, 1], [4, 1], [6, 1], [8, 1], [10, 1], [11, 1]]

  def finalpos(pods, ((x, y), t))
    return false if (x != {"A" => 3, "B" => 5, "C" => 7, "D" => 9}[t])
    return ((y + 1)..@@ymax).all? { |yr| pods[[x, yr]] == t }
  end

  def pathcost(pods, t, (x1, y1), (x2, y2))
    ( (1...y1) .step( 1)               .map { |y| [x1, y] } +
      (x2...x1).step(x2 <= x1 ? 1 : -1).map { |x| [x , 1] } +
      (y2...1) .step(-1)               .map { |y| [x2, y] } )
      .each { |(x, y)| break nil if pods.key?([x, y]) }
      .then { |p|
      ( { "A" => 1, "B" => 10, "C" => 100, "D" => 1000 }[t] *
        p.length) if not p.nil? }
  end

  def possible_moves(pods, ((x, y), t))
    return [] if finalpos(pods, [[x, y], t])
    dx = {"A" => 3, "B" => 5, "C" => 7, "D" => 9}[t]
    ( ( ( y != 1 ) ? HALLWAY : [] ) +
      (@@ymax..2).step(-1).find { |y| pods[[dx, y]] != t }.then {|y| [[dx, y]] }
    ).map { |(xh, yh)|
      cost = pathcost(pods, t, [x, y], [xh, yh])
      [ [xh, yh], cost ] if cost
    }.compact
  end

  def tryall(pods, energy=0)
    @@ymax  ||= pods.keys.map(&:last).max
    @@cache ||= {} # pods -> energy

    match = @@cache.fetch(pods, nil)
    return nil if match and match <= energy
    @@cache[pods] = energy

    return energy if pods.all? { |pod| finalpos(pods, pod) }

    allmoves = pods.map { |pod|
      possible_moves(pods, pod).map { |mov| [pod, mov] }  }.flatten(1)

    allmoves.map { |((x1, y1), t), ((x2, y2), en)|
      newpods = pods.except( [x1, y1] ).update({ [x2, y2] => t })
      tryall(newpods, energy + en)
    }.compact.min
  end
end

# 16506 (took ~50s)
def d21231()
  include D2123
  pods = { [3, 2] => "B", [5, 2] => "A", [7, 2] => "B", [9, 2] => "C",
           [3, 3] => "D", [5, 3] => "A", [7, 3] => "D", [9, 3] => "C" }
  tryall(pods)
end

# 48304 (took ~200s)
def d21232()
  include D2123
  pods = { [3, 2] => "B", [5, 2] => "A", [7, 2] => "B", [9, 2] => "C",
           [3, 3] => "D", [5, 3] => "C", [7, 3] => "B", [9, 3] => "A",
           [3, 4] => "D", [5, 4] => "B", [7, 4] => "A", [9, 4] => "C",
           [3, 5] => "D", [5, 5] => "A", [7, 5] => "D", [9, 5] => "C" }
  tryall(pods)
end


# ###########################################################################
#
# 2021 DAY 22
#
# ###########################################################################

# 587785
def d21221()
  input(2122).split("\n")
    .map { |line| line.scan(/(?:on|off|[-0-9]+)/) }
    .map { |sw, *ps| [sw] + ps.map(&:to_i) }
    .filter { |_, *ps| ps.all? { |c| c.between?(-50, 50) } }
    .tap { |steps| debug("nbsteps", steps.length) }
    .reduce(Set[]) { |acc, (sw, x1, x2, y1, y2, z1, z2)|
      newset = (x1..x2).to_a.product((y1..y2).to_a, (z1..z2).to_a).to_set
      acc = (sw == "on") ? (acc | newset) : (acc - newset)
      debug("len", acc.length)
      acc
    }
    .size
end

# ["on x=0..9,y=0..9,z=0..9",
#  "on x=4..6,y=4..6,z=4..6"]

#  x1 x2 y1 y2 z1 z2
# [ 0, 9, 0, 9, 0, 9 ] break [ 4, 6, 4, 6, 4, 6 ]

# 9 A A A A D D D B B B
# 8 A A A A D D D B B B
# 7 A A A A D D D B B B
# 6 A A A A . . . B B B
# 5 A A A A . . . B B B
# 4 A A A A . . . B B B
# 3 A A A A C C C B B B
# 2 A A A A C C C B B B
# 1 A A A A C C C B B B
# 0 A A A A C C C B B B
# / 0 1 2 3 4 5 6 7 8 9

#y4-6z7-9   A F F F B
#y4-6z4-6  A . . . B
#y4-6z0-3 A E E E B

# [ 0, 3, 0, 9, 0, 9 ] A lowx  (front)
# [ 7, 9, 0, 9, 0, 9 ] B highx (back)
# [ 4, 6, 0, 3, 0, 9 ] C lowy  (below)
# [ 4, 6, 7, 9, 0, 9 ] D highy (above)
# [ 4, 6, 4, 6, 0, 3 ] E lowz  (lowdeep)
# [ 4, 6, 4, 6, 7, 9 ] F highz (hghdeep)

# 1167985679908143
def d21222()
  input(2122).split("\n")
    .map { |line| line.scan(/(?:on|off|[-0-9]+)/) }
    .map { |sw, *ps| [sw] + ps.map(&:to_i) }
    # .filter { |_, *ps| ps.all? { |c| c.between?(-50, 50) } }
    .tap { |steps| debug("nbsteps", steps.length) }
    .reduce([]) { |acc, (sw, nx1, nx2, ny1, ny2, nz1, nz2)|
      # maintain the list of non-overlapping "on" cubes
      newcubes =
        acc.map { |(ox1, ox2, oy1, oy2, oz1, oz2)|
        if ( nx1 > ox2 or nx2 < ox1 or
             ny1 > oy2 or ny2 < oy1 or
             nz1 > oz2 or nz2 < oz1 )
          # old cube does not overlap -> keep it unchanged
          [[ox1, ox2, oy1, oy2, oz1, oz2]]
        else
          # old cube overlaps -> lets do the breaking
          gx1, sx2 = [ox1, nx1].max, [ox2, nx2].min
          gy1, sy2 = [oy1, ny1].max, [oy2, ny2].min
          [
            (nx1 > ox1) ? [ox1  , nx1-1, oy1  , oy2  , oz1  , oz2  ] : nil,
            (nx2 < ox2) ? [nx2+1, ox2  , oy1  , oy2  , oz1  , oz2  ] : nil,
            (ny1 > oy1) ? [gx1  , sx2  , oy1  , ny1-1, oz1  , oz2  ] : nil,
            (ny2 < oy2) ? [gx1  , sx2  , ny2+1, oy2  , oz1  , oz2  ] : nil,
            (nz1 > oz1) ? [gx1  , sx2  , gy1  , sy2  , oz1  , nz1-1] : nil,
            (nz2 < oz2) ? [gx1  , sx2  , gy1  , sy2  , nz2+1, oz2  ] : nil,
          ].compact
        end
        }.flatten(1)
      # add the new cube (now non-overlapping) if it is "on"
      newcubes.push([nx1, nx2, ny1, ny2, nz1, nz2]) if sw == "on"
      newcubes
    }
    .map { |(x1, x2, y1, y2, z1, z2)|
      (x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1)
    }.sum
end


# ###########################################################################
#
# 2021 DAY 21
#
# ###########################################################################

# 679329
def d21211()
  pps = input(2121).split("\n").map{ _1.split.last.to_i }
  psc, die, n = [0, 0], 0, 0
  loop do
    roll = [1, 2, 3].map { (die + _1) % 100}.sum
    die = (die + 3) % 100
    pps[n % 2] = (((pps[n % 2] + roll) - 1) % 10) + 1
    psc[n % 2] = psc[n % 2] + pps[n % 2]
    n = n + 1
    break if psc.max >= 1000
  end
  psc.min * n * 3
end

#
# # total roll for player 1 at round n :
# r1 = ( (( n * (n + 1) * 18 ) / 2 ) + (6 * (n + 1)) )
#

#
# # position of player 1 at round n :
# p1 = ( ( init1 + r1 - 1 ) % 10 ) + 1
#

#
# # score of player 1 (depends on previous round)
# s1 = s1 + p1
#

# 433315766324816 (took ~15s) ugly and slow solution
def d21212()
  def nextplay(play, n)
    play.to_a
      .product( [ [3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1] ] )
      .map { |( (pos1, scr1, pos2, scr2), nbu), (newrol, nbr)|
        if n == 1
          pos1 = ( ( ( pos1 + newrol - 1 ) % 10 ) + 1 )
          scr1 += pos1
        else
          pos2 = ( ( ( pos2 + newrol - 1 ) % 10 ) + 1 )
          scr2 += pos2
        end
        { [ pos1, scr1, pos2, scr2 ] => nbu * nbr } }
      .reduce( {} ) { |acc, nh|
        acc.merge(nh) { |_, oldv, newv| oldv + newv} }
  end

  p1, p2 = input(2121).split("\n").map{ _1.split.last.to_i }
  play = { [ p1, 0, p2, 0 ] => 1 }
  nb1, nb2 = 0, 0

  loop do
    play = nextplay( play, 1 )
    play, endx = play.partition{ |(r1, s1, r2, s2), n| s1 < 21 }
    nb1 += endx.map(&:last).sum

    play = nextplay( play, 2 )
    play, endx = play.partition{ |(r1, s1, r2, s2), n| s2 < 21 }
    nb2 += endx.map(&:last).sum

    break [nb1, nb2].max if play.empty?
  end
end

# cleaner and faster solution
def d2121rec()
  p1, p2 = input(2121).split("\n").map{ _1.split.last.to_i }

  @cache = { }
  def play(p1, s1, p2, s2)
    return [1, 0] if s1 >= 21
    return [0, 1] if s2 >= 21
    match = @cache.fetch([p1, s1, p2, s2], nil)
    return match if match
    [1, 2, 3].product([1, 2, 3], [1, 2, 3])
      .map(&:sum).map { |roll|
        np1 = ( ( ( p1 + roll - 1 ) % 10 ) + 1 )
        play(p2, s2, np1, s1 + np1).reverse }
      .transpose.map(&:sum)
      .tap { |a1, a2| @cache[[p1, s1, p2, s2]] = [a1, a2] }
  end

  play(p1, 0, p2, 0).max
end


# ###########################################################################
#
# 2021 DAY 20
#
# ###########################################################################

module D2120

  def resize(img, siz)
    ((xmin, ymin), (xmax, ymax)) = img.keys().minmax
    ((xmin - siz)..(xmax + siz)).map { |x|
      ((ymin - siz)..(ymax + siz)).map { |y|
        [[x, y], img.fetch([x, y], ".")] }
    }.flatten(1).to_h
  end

  def getimage(lines)
    lines.each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c] }
    }.flatten(1).to_h
  end

  def enhance(enh, img)
    ((xmin, ymin), (xmax, ymax)) = img.keys().minmax
    ((xmin)..(xmax)).map { |x|
      ((ymin)..(ymax)).map { |y|
        [[x - 1, y - 1], [x, y - 1], [x + 1, y - 1],
         [x - 1, y    ], [x, y    ], [x + 1, y    ],
         [x - 1, y + 1], [x, y + 1], [x + 1, y + 1]]
          .map { |nx, ny| img.fetch([nx, ny], ".") }
          .map { |c| {"#" => "1", "." => "0"}[c] }.join.to_i(2)
          .then{ |i| [ [x, y], enh[i] ] }
      } }.flatten(1).to_h
  end

  def tostr(img)
    ((xmin, ymin), (xmax, ymax)) = img.keys().minmax
    (ymin..ymax).map { |y| (xmin..xmax).map { |x| img[[x,y]] }.join }.join("\n")
  end

  def enhloop(enh, img, nbiter, debug=false)
    # not sure if (+3, -1) works for all enhancement lists
    img = resize(img, nbiter + 3)
    (1..nbiter).each { |n|
      img = enhance(enh, img)
      img = resize(resize(img, -1), 1) if n.even?
      puts tostr(img) + "\n\n" if debug }
    img
  end

end

# 5065
def d21201()
  include D2120
  enh, _, *inp = input(2120).split("\n")

  enhloop(enh, getimage(inp), 2)
    .values.filter { _1 == "#" }.length
end

# 14790 (took ~30s)
def d21202()
  include D2120
  enh, _, *inp = input(2120).split("\n")

  enhloop(enh, getimage(inp), 50)
    .values.filter { _1 == "#" }.length
end

# .................  .................  .................  .................
# .................  .................  .................  ...........##....
# .................  .................  ...........#.....  .....##.#.###....
# .................  ......##.##......  .....#..#.#......  ....###..#.#.#...
# ......#..#.......  .....#..#.#......  ....#.#...###....  ...###..#.#.##...
# ......#..........  .....##.#..#.....  ....#...##.#.....  ...####.##.#.....
# ......##..#......  .....####..#.....  ....#.....#.#....  ...###..#...##...
# ........#........  ......#..##......  .....#.#####.....  ....#.#....#.....
# ........###......  .......##..#.....  ......#.#####....  .....##.#..#.#...
# .................  ........#.#......  .......##.##.....  ......#.####.....
# .................  .................  ........###......  .......####......
# .................  .................  .................  ........#.#......
# .................  .................  .................  .................

# .................  #################  .................  #################
# .................  #################  .................  #################
# .................  #################  .................  ######.##########
# .................  #################  .....#.#.........  ####...##.#######
# ......#..#.......  ######...########  ....###.##.......  ###.#.#.#..######
# ......#..........  #####.#.#..######  ........##.......  ####.#....#######
# ......##..#......  ######....#######  ....##.##.#......  ###.#...#..######
# ........#........  ######......#####  ........##.......  ####..####.######
# ........###......  ########...######  .....#...#.......  ####.############
# .................  ########..#######  .........#.......  #########.#######
# .................  #################  .......#.........  ######.##########
# .................  #################  .................  #################
# .................  #################  .................  #################


# ###########################################################################
#
# 2021 DAY 19
#
# ###########################################################################

module D2119

  # if report n0 and report n1 overlap,
  # -> update coordinates from report n1 relatively to n0 (absolute coord)
  # -> return coordinates of scanner n1
  def adjust(reports, n0, n1)
    # 48 configs - should be 24 ???
    orientations =
      [0, 1, 2]
        .permutation(3).to_a
        .product([-1, 1].product([-1, 1], [-1, 1]))
        .map { |a, b| a.zip(b) }
    orientations.each { |(a1, d1), (a2, d2), (a3, d3)|
      # optim? we would still find overlap without trying 11 first points
      reports[n0].each { |x0, y0, z0|
        reports[n1].each { |r1|
          # for each orientation, try every pair of points from r0 & r1
          # if we consider them as the same point, how many other points match ?
          x1, y1, z1 = r1[a1] * d1, r1[a2] * d2, r1[a3] * d3
          xoff, yoff, zoff = x1 + x0, y1 + y0, z1 + z0
          # position of scanner n1 is [xoff, yoff, zoff] ( from scanner n0 pov )
          reports_n1_adjusted = reports[n1].map { |r1|
            [xoff - r1[a1] * d1, yoff - r1[a2] * d2, zoff - r1[a3] * d3] }
          #
          nboverlaps = reports_n1_adjusted.intersection(reports[n0]).length
          next if nboverlaps < 12
          reports[n1] = reports_n1_adjusted
          return xoff, yoff, zoff
        } } }
    false
  end

  # assemble the full map
  # -> update all reports with absolute coordinates (relative to first scanner)
  # -> return list of coords of all scanners
  def assemble(reports)
    n_done, n_todo = [0], reports.each_index.drop(1)
    n_ttry = n_done.dup  # to be tried as ref
    sc_pos = [[0, 0, 0]] # scanner positions
    while not n_todo.empty?
      nref = n_ttry.shift()
      n_todo.dup.each { |n1|
        s1_pos = adjust(reports, nref, n1)
        next unless s1_pos
        n_todo.delete(n1)
        n_ttry.push(n1)
        n_done.push(n1)
        sc_pos.push(s1_pos) }
    end
    sc_pos
  end

  # parse reports of all scanners
  def getreports(input)
    input.split("\n\n")
      .map { |s| s.split("\n") }.map { |_, *lines|
    lines.map { |l| l.split(",").map(&:to_i) } }
  end

end

# 394 (took > 300s !)
def d21191()
  include D2119
  reports = getreports(input(2119))
  assemble(reports)
  reports.flatten(1).uniq.length
end

# 12304
def d21192()
  include D2119
  reports = getreports(input(2119))
  scanpos = assemble(reports)
  scanpos.combination(2).map { |(x1, y1, z1), (x2, y2, z2)|
    (x2 - x1).abs + (y2 - y1).abs + (z2 - z1).abs }.max
end


# ###########################################################################
#
# 2021 DAY 18
#
# ###########################################################################

module D2118

  # ##################################################################

  # explodes(inp)
  #   modifies its argument by performing first explosion
  #   returns true if something changed, false otherwise

  def explode(inp, elt = inp, crd = [])
    case elt
    in [Integer => e0, Integer => e1] if crd.length >= 4 then
      # on a deep pair of numbers -> explode criteria met

      def allnums(elt, crd = [])
        return [crd] if elt.is_a? Integer
        allnums(elt[0], crd + [0]) + allnums(elt[1], crd + [1])
      end
      allnums = allnums( inp )

      # update first number on the right
      idx = allnums.index(crd + [0]) - 1
      if (0...allnums.length).include?(idx)
        *hd, ls = allnums[idx]
        hd.reduce(inp, :fetch)[ls] += e0
      end

      # update first number on the left
      idx = allnums.index(crd + [1]) + 1
      if (0...allnums.length).include?(idx)
        *hd, ls = allnums[idx]
        hd.reduce(inp, :fetch)[ls] += e1
      end

      # replace exploding pair by number 0
      *hd, ls = crd
      hd.reduce(inp, :fetch)[ls] = 0
      return true # list was modified

    in Integer then
      # on a regular number -> nothing to do
      return false # nothing changed

    in [e0, e1] then
      # on a pair -> look deeper
      return (explode(inp, e0, crd + [0]) or explode(inp, e1, crd + [1]))
    end
  end

  # ##################################################################

  # split(inp)
  #   modifies its argument by performing first split
  #   returns true if something changed, false otherwise

  def split(inp, elt = inp, crd = [])
    case elt
    in Integer => val if val >= 10 then
      # on a number >= 10 -> split criteria met
      # insert the new pair
      *hd, ls = crd
      hd.reduce(inp, :fetch)[ls] = [val.fdiv(2).floor, val.fdiv(2).ceil]
      return true

    in Integer then
      # on another integer -> nothing to do
      return false

    in [e0, e1] then
      # on a pair -> continue digging
      return (split(inp, e0, crd + [0]) or split(inp, e1, crd + [1]))
    end
  end

  # ##################################################################

  # addred(lst)
  #   returns reduced addition of given snailfish numbers in lst
  #   ( does not modify anything )

  def addred(lst)

    def deepcp(elt)
      case elt
        in [_, _] then elt.map{ deepcp _1 }
        in _      then elt
      end
    end

    def reduce(inp)
      loop do
        next if explode(inp)
        next if split(inp)
        return inp
      end
    end

    # lst must be copied first as it will be modified by explode & split
    deepcp(lst).reduce { |acc, obj| reduce([acc, obj]) }
  end

  # ##################################################################

  def magnitude(elt)
    case elt
    in Integer  then elt
    in [e0, e1] then
      3 * magnitude(e0) + 2 * magnitude(e1)
    end
  end

  # ##################################################################
end

# 3699
def d21181()
  include D2118
  input(2118).split("\n")
    .then { |l| eval("[%s]" % l.join(",")) }
    .then { |l| magnitude(addred(l)) }
end

# 4735
def d21182()
  include D2118
  input(2118).split("\n")
    .then { |l| eval("[%s]" % l.join(",")) }
    .then { |l|
      l.combination(2).map{ |a, b|
        [ magnitude(addred([a, b])),
          magnitude(addred([b, a])) ]
      }.flatten.max }
end

def d2118unit()
  include D2118

  def testfunc(fct, inp) send(fct, inp); inp end

  # ##################################################################
  # explode
  fail unless testfunc(:explode, [[[[[9,8],1],2],3],4]) == [[[[0,9],2],3],4]
  fail unless testfunc(:explode, [[[[[9,8],1],2],3],4]) == [[[[0,9],2],3],4]
  fail unless testfunc(:explode, [7,[6,[5,[4,[3,2]]]]]) == [7,[6,[5,[7,0]]]]
  fail unless testfunc(:explode, [[6,[5,[4,[3,2]]]],1]) == [[6,[5,[7,0]]],3]
  fail unless testfunc(:explode, [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]) ==
              [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
  fail unless testfunc(:explode, [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]) ==
              [[3,[2,[8,0]]],[9,[5,[7,0]]]]
  fail unless testfunc(:explode, [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]) ==
              [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
  fail unless testfunc(:explode, [[[[0,7],4],[7,[[8,4],9]]],[1,1]]) ==
              [[[[0,7],4],[15,[0,13]]],[1,1]]
  fail unless testfunc(:explode, [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]) ==
              [[[[0,7],4],[[7,8],[6,0]]],[8,1]]

  # ##################################################################
  # split
  fail unless testfunc(:split, [[[[0,7],4],[15,[0,13]]],[1,1]]) ==
              [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
  fail unless testfunc(:split, [[[[0,7],4],[[7,8],[0,13]]],[1,1]]) ==
              [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]

  # ##################################################################
  # addred
  fail unless addred([[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]) ==
              [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
  fail unless addred([[1,1],[2,2],[3,3],[4,4]]) ==
              [[[[1,1],[2,2]],[3,3]],[4,4]]
  fail unless addred([[1,1],[2,2],[3,3],[4,4],[5,5]]) ==
              [[[[3,0],[5,3]],[4,4]],[5,5]]
  fail unless addred([[1,1],[2,2],[3,3],[4,4],[5,5],[6,6]]) ==
              [[[[5,0],[7,4]],[5,5]],[6,6]]
  fail unless addred([ [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],
                       [7,[[[3,7],[4,3]],[[6,3],[8,8]]]],
                       [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]],
                       [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]],
                       [7,[5,[[3,8],[1,4]]]],
                       [[2,[2,2]],[8,[8,1]]],
                       [2,9],
                       [1,[[[9,3],9],[[9,0],[0,7]]]],
                       [[[5,[7,4]],7],1],
                       [[[[4,2],2],6],[8,7]] ] ) ==
              [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
  fail unless addred([ [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]],
                       [[[5,[2,8]],4],[5,[[9,9],0]]],
                       [6,[[[6,2],[5,6]],[[7,6],[4,7]]]],
                       [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]],
                       [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]],
                       [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]],
                       [[[[5,4],[7,7]],8],[[8,3],8]],
                       [[9,3],[[9,9],[6,[4,9]]]],
                       [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],
                       [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]] ]) ==
              [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]

  # ##################################################################
  # magnitude
  fail unless magnitude([9,1]) == 29
  fail unless magnitude([1,9]) == 21
  fail unless magnitude([[9,1],[1,9]]) == 129
  fail unless magnitude([[1,2],[[3,4],5]]) == 143.
  fail unless magnitude([[[[0,7],4],[[7,8],[6,0]]],[8,1]]) == 1384.
  fail unless magnitude([[[[1,1],[2,2]],[3,3]],[4,4]]) == 445.
  fail unless magnitude([[[[3,0],[5,3]],[4,4]],[5,5]]) == 791.
  fail unless magnitude([[[[5,0],[7,4]],[5,5]],[6,6]]) == 1137.
  fail unless magnitude(
                [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]) == 3488
  fail unless magnitude(
                [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
              ) == 4140
  fail unless magnitude(
                addred([ [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],
                         [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]] ])
              ) == 3993
end


# ###########################################################################
#
# 2021 DAY 17
#
# ###########################################################################

# "target area: x=241..275, y=-75..-49"
# "target area: x=20..30, y=-10..-5"

def shoot2117(vx, vy, x1, x2, y1, y2)
  x, y, ymax = 0, 0, 0
  loop do
    return ymax if x.between?(x1, x2) and y.between?(y1, y2)
    return nil if x > x2 or y < y1
    x, y = x + vx, y + vy
    ymax = ymax > y ? ymax : y
    vx += (vx > 0) ? -1 : (vx == 0) ? 0 : fail
    vy -= 1
  end
end

# 2775
def d21171()
  x1, x2, y1, y2 = input(2117).scan(/[-0-9]+/).map(&:to_i)

  (0..x2).map { |vx| (y1..100).map { |vy|
    shoot2117(vx, vy, x1, x2, y1, y2) }.compact }.flatten.max
end

# 1566
def d21172()
  x1, x2, y1, y2 = input(2117).scan(/[-0-9]+/).map(&:to_i)

  (0..x2).map { |vx| (y1..100).map { |vy|
    shoot2117(vx, vy, x1, x2, y1, y2) }.compact }.flatten.length
end


# ###########################################################################
#
# 2021 DAY 16
#
# ###########################################################################

def getpack2116()
  def getpack(bits)
    pck_ver = bits.shift(3).join.to_i(2)
    pck_typ = bits.shift(3).join.to_i(2)
    if pck_typ == 4 then # literal
      pck_val = []
      loop do
        keep = bits.shift(1).first
        pck_val.push( bits.shift(4) )
        break if keep.zero?
      end
      pck_val = pck_val.join.to_i(2)
      {ver: pck_ver, typ: pck_typ, val: pck_val}
    else # operator
      pck_lty = bits.shift(1).first
      if pck_lty.zero?
        sub_bln = bits.shift(15).join.to_i(2)
        sub_bts = bits.shift(sub_bln)
        sub_pks = []
        while not sub_bts.empty?
          sub_pks.push( getpack(sub_bts) )
        end
      else
        pck_nsb = bits.shift(11).join.to_i(2)
        sub_pks = pck_nsb.times.map { getpack(bits) }
      end
      {ver: pck_ver, typ: pck_typ, subs: sub_pks}
    end
  end

  input(2116)
    .chars.map { |c| c.to_i(16).to_s(2).rjust(4, "0").chars.map(&:to_i) }
    .flatten
    .then { |bits| getpack(bits) }
end

# 934
def d21161()
  def evalpack(pck)
    pck[:ver] + (pck[:subs] || []).map { |sub| evalpack(sub) }.sum
  end

  evalpack(getpack2116())
end

# Packets with type ID 0 are sum packets
# - their value is the sum of the values of their sub-packets.
#   If they only have a single sub-packet, their value is the value of the sub-packet.
# Packets with type ID 1 are product packets
# - their value is the result of multiplying together the values of their sub-packets.
#   If they only have a single sub-packet, their value is the value of the sub-packet.
# Packets with type ID 2 are minimum packets
# - their value is the minimum of the values of their sub-packets.
# Packets with type ID 3 are maximum packets
# - their value is the maximum of the values of their sub-packets.
# Packets with type ID 5 are greater than packets
# - their value is 1 if the value of the first sub-packet is greater than
#   the value of the second sub-packet; otherwise, their value is 0.
#   These packets always have exactly two sub-packets.
# Packets with type ID 6 are less than packets
# - their value is 1 if the value of the first sub-packet is less than
#   the value of the second sub-packet; otherwise, their value is 0.
#   These packets always have exactly two sub-packets.
# Packets with type ID 7 are equal to packets
# - their value is 1 if the value of the first sub-packet is equal to
#   the value of the second sub-packet; otherwise, their value is 0.
#   These packets always have exactly two sub-packets.

# 912901337844
def d21162()
  def evalpack(pck)
    case pck[:typ]
    when 4 then # literal
      pck[:val]
    when 0 then # sum
      pck[:subs].map { |s| evalpack(s) }.reduce(:+)
    when 1 then # product
      pck[:subs].map { |s| evalpack(s) }.reduce(:*)
    when 2 then # minimum
      pck[:subs].map { |s| evalpack(s) }.min
    when 3 then # maximum
      pck[:subs].map { |s| evalpack(s) }.max
    when 5 then # greater than
      sub1, sub2 = pck[:subs].map { |s| evalpack(s) }
      ( sub1 > sub2 ) ? 1 : 0
    when 6 then # less than
      sub1, sub2 = pck[:subs].map { |s| evalpack(s) }
      ( sub1 < sub2 ) ? 1 : 0
    when 7 then # less than
      sub1, sub2 = pck[:subs].map { |s| evalpack(s) }
      ( sub1 == sub2 ) ? 1 : 0
    else
      fail
    end
  end

  evalpack(getpack2116())
end


# ###########################################################################
#
# 2021 DAY 15
#
# ###########################################################################

# 745
def d21151()
  cavern =
    input(2115).split("\n").each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c.to_i]}
    }.flatten(1).to_h

  visited, tovisit, dest = Set[], {[0, 0] => 0}, cavern.keys.max
  while true
    pos, cst = tovisit.sort_by(&:last).first
    return cst if pos == dest
    tovisit.delete(pos)
    visited.add(pos)
    nexts =
      pos.then { |x, y| [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] }
        .filter { |npos| cavern.include? npos}
        .filter { |npos| not visited.include? npos }
        .map { |npos| [npos, cst + cavern[npos]] }.to_h
    tovisit.update(nexts) { |_, old, new| (old <= new) ? old : new }
  end
end

# 3002
def d21152()
  cavern =
    input(2115).split("\n").each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c.to_i]}
    }.flatten(1).to_h

  cmax, nmul = cavern.keys.max.first, 5
  cavern =
    (0...nmul).map { |nx| (0...nmul).map { |ny|
      cavern.map { |(x, y), v|
        [ [nx * (cmax + 1) + x, ny * (cmax + 1) + y],
          ((v - 1 + nx + ny) % 9) + 1 ]
      } } }.flatten(2).to_h

  visited, tovisit, dest = Set[], {[0, 0] => 0}, cavern.keys.max
  while true
    pos, cst = tovisit.sort_by(&:last).first
    return cst if pos == dest
    tovisit.delete(pos)
    visited.add(pos)
    nexts =
      pos.then { |x, y| [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] }
        .filter { |npos| cavern.include? npos}
        .filter { |npos| not visited.include? npos }
        .map { |npos| [npos, cst + cavern[npos]] }.to_h
    tovisit.update(nexts) { |_, old, new| (old <= new) ? old : new }
  end
end

# [1] 1  6  3  7  5  1  7  4  2
# [1] 3  8  1  3  7  3  6  7  2
# [2][1][3][6][5][1][1] 3  2  8
#  3  6  9  4  9  3 [1][5] 6  9
#  7  4  6  3  4  1  7 [1][1] 1
#  1  3  1  9  1  2  8  1 [3] 7
#  1  3  5  9  9  1  2  4 [2] 1
#  3  1  2  5  4  2  1  6 [3] 9
#  1  2  9  3  1  3  8  5 [2][1]
#  2  3  1  1  9  4  4  5  8 [1]

def d2115viz()
  cavern =
    input(2115.0).split("\n").each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c.to_i]}
    }.flatten(1).to_h

  cmax, nmul = cavern.keys.max.first, 5
  cavern =
    (0...nmul).map { |nx| (0...nmul).map { |ny|
      cavern.map { |(x, y), v|
        [ [nx * (cmax + 1) + x, ny * (cmax + 1) + y],
          ((v - 1 + nx + ny) % 9) + 1 ]
      } } }.flatten(2).to_h

  visited, tovisit, dest = Set[], {[0, 0] => [0, [[0, 0]]]}, cavern.keys.max
  cmax = cavern.keys.max.first
  while true
    pos, (cst, path) = tovisit.sort_by {|k, (c, p)| c}.first
    puts "\033[?25l\033[#{cmax + 1}A" +
         (0..cmax).map { |y| (0..cmax).map { |x|
           color = path.include?([x, y])?31:visited.include?([x, y])?0:30
           "\033[#{color};1m#{cavern[[x, y]]}\033[0m"
    }.join }.join("\n") + "\033[?25h"
    return cst if pos == dest
    tovisit.delete(pos)
    visited.add(pos)
    nexts =
      pos.then { |x, y| [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] }
        .filter { |npos| cavern.include? npos}
        .filter { |npos| not visited.include? npos }
        .map { |npos| [npos, [cst + cavern[npos], path + [npos]]] }.to_h
    tovisit.update(nexts) { |_, (old, po), (new, pn)|
      (old <= new) ? [old, po] : [new, pn] }
  end
end


# ###########################################################################
#
# 2021 DAY 14
#
# ###########################################################################

# 2937
def d21141()
  tmpl, ruls = input(2114).split("\n\n").then { |t, r|
    [t, r.scan(/(\w+) -> (\w+)/).to_h] }

  (1..10).reduce(tmpl) { |tmp, _|
    tmp.chars.each_cons(2)
      .map { |p| [p.first, ruls[p.join], p.last] }
      .reduce { |acc, obj| acc + obj.drop(1) }
      .join }
    .chars.tally.values.minmax.then { |min, max| max - min }
end

# NNCB
# CH -> B  HH -> N  CB -> H  NH -> C
# HB -> C  HC -> B  HN -> C  NN -> C
# BH -> H  NC -> B  NB -> B  BN -> B
# BB -> N  BC -> B  CC -> N  CN -> C

# NNCB
#   {["N", "N"]=>1, ["N", "C"]=>1, ["C", "B"]=>1}
# NCNBCHB
#   {"N" => 1} + {"N"=>1, "C"=>2, "B"=>1, "H"=>1}
#   {["N", "C"]=>1, ["C", "N"]=>1, ["N", "B"]=>1, ["B", "C"]=>1, ["C", "H"]=>1, ["H", "B"]=>1}
# NBCCNBBBCBHCB
#   {"N" => 1} + {"N"=>2, "B"=>6, "C"=>3, "H"=>1}
# NBBBCNCCNBBNBNBBCHBHHBCHB

# 3390034818249
def d21142()
  tmpl, ruls = input(2114).split("\n\n").then { |t, r|
    [t, r.scan(/(\w+) -> (\w+)/).to_h] }

  frst = tmpl.chars.first
  pcnt = tmpl.chars.each_cons(2).tally

  (1..40)
    # compute new pairs count
    .reduce(pcnt) { |pcnt, _|
      pcnt
      .map { |(c1, c2), n| ci = ruls[c1 + c2]; {[c1, ci] => n, [ci, c2] => n} }
      .reduce({}) { |acc, cnt| acc.merge(cnt) { |_, old, new| old + new } }
    }
    # once finished, compute chars count (first char + last char of pairs)
    .reduce({ frst => 1 }) { |acc, ((_, c), n)|
      acc.merge( { c => n } ) { |_, old, new| old + new }
    }
    .values.minmax.then { |min, max| max - min }
end


# ###########################################################################
#
# 2021 DAY 13
#
# ###########################################################################

# 682
def d21131()
  dots, flds = input(2113).split("\n\n").then { |d, f|
    [ d.scan(/(\d+),(\d+)/).map { |x, y| [x.to_i, y.to_i ] },
      f.scan(/(\w+)=(\d+)/).map { |c, v| [c, v.to_i] } ] }

  flds.take(1)
    .reduce(dots) { |acc, (ax, vl)|
      acc.map { |x, y|
        ( ax == "x" ) ? ( (x < vl) ? [x, y] : [vl * 2 - x, y] ) :
        ( ax == "y" ) ? ( (y < vl) ? [x, y] : [x, vl * 2 - y] ) : fail
      }.uniq
  }.length
end

# FAGURZHE
def d21132()
  dots, flds = input(2113).split("\n\n").then { |d, f|
    [ d.scan(/(\d+),(\d+)/).map { |x, y| [x.to_i, y.to_i ] },
      f.scan(/(\w+)=(\d+)/).map { |c, v| [c, v.to_i] } ] }

  flds
    .reduce(dots) { |acc, (ax, vl)|
      acc.map { |x, y|
        ( ax == "x" ) ? ( (x < vl) ? [x, y] : [vl * 2 - x, y] ) :
        ( ax == "y" ) ? ( (y < vl) ? [x, y] : [x, vl * 2 - y] ) : fail
      }.uniq }
    .then { |dots|
      xmax, ymax = dots.map(&:first).max, dots.map(&:last).max
      (0..ymax).map{ |y| (0..xmax).map{ |x|
        dots.include?([x, y]) ? "#" : " " }.join(" ") }.join("\n") }
    .then { puts _1 }
end

#  # # # #     # #       # #     #     #   # # #     # # # #   #     #   # # # #
#  #         #     #   #     #   #     #   #     #         #   #     #   #
#  # # #     #     #   #         #     #   #     #       #     # # # #   # # #
#  #         # # # #   #   # #   #     #   # # #       #       #     #   #
#  #         #     #   #     #   #     #   #   #     #         #     #   #
#  #         #     #     # # #     # #     #     #   # # # #   #     #   # # # #


# ###########################################################################
#
# 2021 DAY 12
#
# ###########################################################################

# 5228
def d21121()
  @segments = input(2112).split("\n")
      .map{ _1.split("-") }.map { [_1, _1.reverse] }.flatten(1)

  def visit(path, cave)
    return [path] if cave == "end"
    sseen = path.map(&:last).filter{ _1.match?(/^[[:lower:]]/) }
    nexts = @segments
      .filter{ |s, _| s == cave }
      .filter{ |_, e| e != "start" }
      .filter{ |_, e| not sseen.include? e }
    nexts.map { |seg| visit(path + [seg], seg.last) }.flatten(1)
  end

  visit([], "start").length
end

# 131228
def d21122()
  @segments = input(2112).split("\n")
      .map{ _1.split("-") }.map { [_1, _1.reverse] }.flatten(1)

  def visit(path, cave)
    return [path] if cave == "end"
    sseen = path.map(&:last).filter{ _1.match?(/^[[:lower:]]/) }
    nexts = @segments
      .filter{ |s, _| s == cave }
      .filter{ |_, e| e != "start" }
      .filter{ |_, e| (sseen + [e]).tally.values.count(2) <= 1 }
      .filter{ |_, e| (sseen + [e]).tally.values.max <= 2 }
    nexts.map { |seg| visit(path + [seg], seg.last) }.flatten(1)
  end

  visit([], "start").length
end


# ###########################################################################
#
# 2021 DAY 11
#
# ###########################################################################

# 5 4 8 3 1 4 3 2 2 3
# 2 7 4 5 8 5 4 7 1 1
# 5 2 6 4 5 5 6 1 7 3
# 6 1 4 1 3 3 6 1 4 6
# 6 3 5 7 3 8 5 4 7 8
# 4 1 6 7 5 2 4 6 4 5
# 2 1 7 6 8 4 1 7 2 1
# 6 8 8 2 8 8 1 1 3 4
# 4 8 4 6 8 4 8 5 5 4
# 5 2 8 3 7 5 1 5 2 6

# 1673
def d21111()
  @grid =
    input(2111).split("\n").each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c.to_i]}
    }.flatten(1).to_h

  def neighs((x, y))
    [[x - 1, y - 1], [x, y - 1], [x + 1, y - 1],
     [x - 1, y    ],             [x + 1, y    ],
     [x - 1, y + 1], [x, y + 1], [x + 1, y + 1]] & @grid.keys
  end

  def update(xy)
    @grid[xy] = @grid[xy] + 1
    neighs(xy).each{ update(_1) } if @grid[xy] == 10
  end

  def flash(xy)
    @grid[xy] = 0 if @grid[xy] >= 10
  end

  (1..100).map {
    @grid.each_key { |xy| update(xy) }
    @grid.each_key { |xy| flash(xy) }
    # debug("\n" + (0..9).map{ |y| (0..9).map { |x|
    #    @grid[[x, y]] }.join(" ") }.join("\n") )
    @grid.each_value.filter { _1 == 0 }.length
  }.sum
end

# 279
def d21112()
  @grid =
    input(2111).split("\n").each_with_index.map { |l, y|
      l.chars.each_with_index.map { |c, x| [[x, y], c.to_i]}
    }.flatten(1).to_h

  def neighs((x, y))
    [[x - 1, y - 1], [x, y - 1], [x + 1, y - 1],
     [x - 1, y    ],             [x + 1, y    ],
     [x - 1, y + 1], [x, y + 1], [x + 1, y + 1]] & @grid.keys
  end

  def update(xy)
    @grid[xy] = @grid[xy] + 1
    neighs(xy).each{ update(_1) } if @grid[xy] == 10
  end

  def flash(xy)
    @grid[xy] = 0 if @grid[xy] >= 10
  end

  (1..).each { |n|
    @grid.each_key { |xy| update(xy) }
    @grid.each_key { |xy| flash(xy) }
    break n if @grid.all? { |_, v| v == 0 }
  }
end


# ###########################################################################
#
# 2021 DAY 10
#
# ###########################################################################

# <<~END
#   {([(<{}[<>[]}>{[]{[(<()>
#   [[<[([]))<([[{}[[()]]]
#   [{[{({}]{}}([{[{{{}}([]
#   [<(<(<(<{}))><([]([]()
#   <{([([[(<>()){}]>(<<{{
# END

# 319233
def d21101()
  input(2110).split("\n")
    .map { |line|
      line.chars.reduce([]) { |acc, chr|
        if acc.last == chr then acc.pop(); acc else
          case chr
          when "(" then acc.push(")")
          when "[" then acc.push("]")
          when "{" then acc.push("}")
          when "<" then acc.push(">")
          when ")" then break 3
          when "]" then break 57
          when "}" then break 1197
          when ">" then break 25137
          else break 0 end
        end
      }
    }.filter { _1.class == Integer }.sum
end

# <<~END
#   [({(<(())[]>[[{[]{<()<>>
#   [(()[<>])]({[<{<<[]>>(
#   (((({<>}<{<{<>}{[]{[]{}
#   {<[[]]>}<{[{[{[]{()[[[]
#   <{([{{}}[<[[[<>{}]]]>[]]
# END

# 1118976874
def d21102()
  input(2110).split("\n")
    .map { |line|
      line.chars.reduce([]) { |acc, chr|
        close = {"(" => ")", "[" => "]", "{" => "}", "<" => ">"}
        if acc.first == chr   then acc.drop(1)
        elsif close.key?(chr) then acc.unshift(close[chr])
        else break nil end
      } }
    .compact
    .map { |chars|
      chars.reduce(0) { |acc, chr|
        (5 * acc) + {")" => 1, "]" => 2, "}" => 3, ">" => 4 }[chr]
    } }.sort.tap { |lst| break lst[lst.length / 2] }
end


# ###########################################################################
#
# 2021 DAY 9
#
# ###########################################################################

# 522
def d21091()
  area = input(2109)
    .split("\n").map { |l| l.chars.map(&:to_i) }

  xmax, ymax = area.first.length, area.length
  area = (0...ymax).map {
    |y| (0...xmax).map { |x| [[x, y], area[y][x]] } }.flatten(1).to_h

  def neighs(area, (x, y))
    [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]
    ].filter { |nx, ny| area.fetch([nx, ny], nil) }
  end

  area.keys.map { |c|
    (area[c] < neighs(area, c).map {|n| area[n]}.min) ? area[c] + 1 : nil }
    .compact.sum
end

# 916688
def d21092()
  area = input(2109)
    .split("\n").map { |l| l.chars.map(&:to_i) }

  xmax, ymax = area.first.length, area.length
  area = (0...ymax).map {
    |y| (0...xmax).map { |x| [[x, y], area[y][x]] } }.flatten(1).to_h

  def neighs(area, (x, y))
    [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]
    ].filter { |nx, ny| area.fetch([nx, ny], nil) }
  end

  area.keys.map { |c| # -> lows
    (area[c] < neighs(area, c).map {|n| area[n]}.min) ? c : nil }.compact
    .map { |low| # -> super-basins
      visited, tovisit = [], [low]
      while not tovisit.empty?
        current = tovisit.shift
        visited.push(current)
        tovisit.push(*neighs(area, current).filter { |n|
          not (visited.include?(n) or tovisit.include?(n)) and
            (area[current]...9).include?(area[n]) })
      end; visited.length
    }.sort.reverse.take(3).reduce(&:*)
end


# ###########################################################################
#
# 2021 DAY 8
#
# ###########################################################################

# 473
def d21081()
  input(2108)
    .split("\n")
    .map { |line| line.split("|").map(&:split) }
    .map { |sig, out| out.map(&:length).filter { [2,3,4,7].include? _1 } }
    .flatten.length
end

#   0:      1:      2:      3:      4:     5:      6:      7:      8:      9:
#  aaaa    ....    aaaa    aaaa    ....   aaaa    aaaa    aaaa    aaaa    aaaa
# b    c  .    c  .    c  .    c  b    c  b    .  b    .  .    c  b    c  b    c
# b    c  .    c  .    c  .    c  b    c  b    .  b    .  .    c  b    c  b    c
#  ....    ....    dddd    dddd    dddd   dddd    dddd    ....    dddd    dddd
# e    f  .    f  e    .  .    f  .    f  .    f  e    f  .    f  e    f  .    f
# e    f  .    f  e    .  .    f  .    f  .    f  e    f  .    f  e    f  .    f
#  gggg    ....    gggg    gggg    ....   gggg    gggg    ....    gggg    gggg

# XXXLEN  1 -> len 2 -      c     f
# XXXLEN  7 -> len 3 -  a   c     f
# XXXLEN  4 -> len 4 -    b c d   f
# XXXLEN  8 -> len 7 -  a b c d e f g
# LNSUP7  3 -> len 5 -  a   c d   f g  (SUP7: acf)
# LNDIF4  2 -> len 5 -  a   c d e   g  (DIF4: aeg)
# LASTLN  5 -> len 5 -  a b   d   f g
# LNSUP4  9 -> len 6 -  a b c d   f g  (SUP4: bcdf)
# LNNOT5  0 -> len 6 -  a b c   e f g  (NOT5: d)
# LASTLN  6 -> len 6 -  a b   d e f g

# 1097568
def d21082()
  input(2108)
    .split("\n")
    .map { |ln| ln.split("|").map(&:split).map { |w| w.map{ _1.chars.sort } } }
    .map { |sig, out|
      vals = {}
      vals[1] = sig.find{ _1.length == 2 }
      vals[7] = sig.find{ _1.length == 3 }
      vals[4] = sig.find{ _1.length == 4 }
      vals[8] = sig.find{ _1.length == 7 }
      vals[3] = sig.find{ _1.length == 5 and (vals[7] - _1).empty? }
      vals[2] = sig.find{ _1.length == 5 and (_1 - vals[4]).length == 3 }
      vals[5] = sig.find{ _1.length == 5 and vals[2] != _1 and vals[3] != _1 }
      vals[9] = sig.find{ _1.length == 6 and (vals[4] - _1).empty? }
      vals[0] = sig.find{ _1.length == 6 and not (vals[5] - _1).empty? }
      vals[6] = sig.find{ _1.length == 6 and vals[0] != _1 and vals[9] != _1 }
      invv = vals.map { |k, v| [v, k] }.to_h
      out.map { |lst| invv[lst].to_s }.join.to_i
    }.sum
end


# ###########################################################################
#
# 2021 DAY 7
#
# ###########################################################################

# 336040
def d21071()
  inlst =
    input(2107).split(/[^0-9]/).map(&:to_i)
  inlst
    .max.times
    .map { |x| inlst.map { |y| (x - y).abs }.sum }
    .min
end

# 94813675
def d21072()
  inlst =
    input(2107).split(/[^0-9]/).map(&:to_i)
  inlst
    .max.times
    .map { |x| inlst.map { |y| (x - y).abs.downto(1).sum }.sum }
    .min
end


# ###########################################################################
#
# 2021 DAY 6
#
# ###########################################################################

# 362639
def d21061()
  def nextgen(g)
    ( g.map{ _1 == 0 ? 6 : _1 - 1 } +  # next timers
      g.filter { _1 == 0 }.map { 8 } ) # new fishes
  end

  initgen = input(2106).split(/[^0-9]/).map(&:to_i)
  (0...80).reduce(initgen) { |acc, _| nextgen(acc) }.length
end

#  1 [124] [ 43] [ 33] [ 55] [ 45] [  0] [  0] [  0] [  0]
#  2 [ 43] [ 33] [ 55] [ 45] [  0] [  0] [124] [  0] [124]
#  3 [ 33] [ 55] [ 45] [  0] [  0] [124] [ 43] [124] [ 43]
#  4 [ 55] [ 45] [  0] [  0] [124] [ 43] [157] [ 43] [ 33]
#  5 [ 45] [  0] [  0] [124] [ 43] [157] [ 98] [ 33] [ 55]
#  6 [  0] [  0] [124] [ 43] [157] [ 98] [ 78] [ 55] [ 45]
#  7 [  0] [124] [ 43] [157] [ 98] [ 78] [ 55] [ 45] [  0]
#  8 [124] [ 43] [157] [ 98] [ 78] [ 55] [ 45] [  0] [  0]
#  9 [ 43] [157] [ 98] [ 78] [ 55] [ 45] [124] [  0] [124]
# 10 [157] [ 98] [ 78] [ 55] [ 45] [124] [ 43] [124] [ 43]
# 11 [ 98] [ 78] [ 55] [ 45] [124] [ 43] [281] [ 43] [157]
# 12 [ 78] [ 55] [ 45] [124] [ 43] [281] [141] [157] [ 98]
# 13 [ 55] [ 45] [124] [ 43] [281] [141] [235] [ 98] [ 78]
# 14 [ 45] [124] [ 43] [281] [141] [235] [153] [ 78] [ 55]
# 15 [124] [ 43] [281] [141] [235] [153] [123] [ 55] [ 45]
# 16 [ 43] [281] [141] [235] [153] [123] [179] [ 45] [124]
# 17 [281] [141] [235] [153] [123] [179] [ 88] [124] [ 43]
# 18 [141] [235] [153] [123] [179] [ 88] [405] [ 43] [281]

# 1639854996917
def d21062()
  def nextgen(g)
    ( # decreased timers
      g.map { |v, n| (v != 0) ? [v - 1, n] : nil }.compact.to_h
    ).merge( # reset timers and new fishes
      g.map { |v, n| (v == 0) ? [[6, n], [8, n]] : nil }.compact.flatten(1).to_h
    ) { |_, old, new| old + new }
  end

  initgen = input(2106).split(/[^0-9]/).map(&:to_i).tally
  (0...256).reduce(initgen) { |acc, _| nextgen(acc) }.values.sum
end

#   # print("\033[A")
#   print((0..8).map {|x| "[%3s] " % g.fetch(x, 0)}.join )


# ###########################################################################
#
# 2021 DAY 5
#
# ###########################################################################

# 6397
def d21051()
  input(2105)
    .split("\n")
    .map { |line| line.scan(/(\d+)/).map{_1.first.to_i} }
    .reduce([]) { |acc, (x1, y1, x2, y2)|
      acc.concat(
        if x1 == x2 then
          (y1..y2).step(y1 <= y2 ? 1 : -1).map { |y| [x1, y] }
        elsif y1 == y2 then
          (x1..x2).step(x1 <= x2 ? 1 : -1).map { |x| [x, y1] }
        else [] end )
    }
    .tally
    .filter { |x, y| y >= 2 }.length
end

# 22335
def d21052()
  input(2105)
    .split("\n")
    .map { |line| line.scan(/(\d+)/).map{_1.first.to_i} }
    .reduce([]) { |acc, (x1, y1, x2, y2)|
      acc.concat(
        if x1 == x2 then
          (y1..y2).step(y1 <= y2 ? 1 : -1).map { |y| [x1, y] }
        elsif y1 == y2 then
          (x1..x2).step(x1 <= x2 ? 1 : -1).map { |x| [x, y1] }
        else
          stepx = (x1 == x2) ? 0 : (x1 <= x2 ? 1 : -1)
          stepy = (y1 == y2) ? 0 : (y1 <= y2 ? 1 : -1)
          (0..((x2 - x1) / stepx)).map { |d| [x1 + d * stepx, y1 + d * stepy] }
        end )
    }
    .tally
    .filter { |x, y| y >= 2 }.length
end


# ###########################################################################
#
# 2021 DAY 4
#
# ###########################################################################

# 33462
def d21041()
  nums, *brds = input(2104).split("\n\n")
  nums = nums.split(",").map(&:to_i)
  brds = brds.map { _1.split.map(&:to_i) }

  def iswin(brd, nums)
    ( brd.each_slice(5).to_a + # rows
      brd.each_slice(5).to_a.transpose # cols
    ).detect { |values| values.difference(nums).empty? }
  end

  nums.reduce([]) { |acc, num|
    winbrd = brds.filter{ |brd| iswin(brd, acc) }.first
    break winbrd.difference(acc).sum * acc.last if winbrd
    acc + [num]
  }
end

# 30070
def d21042()
  nums, *brds = input(2104).split("\n\n")
  nums = nums.split(",").map(&:to_i)
  brds = brds.map { _1.split.map(&:to_i) }

  def iswin(brd, nums)
    ( brd.each_slice(5).to_a + # rows
      brd.each_slice(5).to_a.transpose # cols
    ).detect { |values| values.difference(nums).empty? }
  end

  nums.reduce([brds, []]) { |(rembrds, acc), num|
    newrems = rembrds.difference(rembrds.filter{ |brd| iswin(brd, acc) })
    break rembrds.last.difference(acc).sum * acc.last if newrems.empty?
    [newrems, acc + [num]]
  }
end


# ###########################################################################
#
# 2021 DAY 3
#
# ###########################################################################

# 1071734
def d21031()
  input(2103)
    .split.map(&:chars)
    .transpose.map(&:tally).map { _1.sort_by(&:last).map(&:first) }
    .transpose.map { _1.join.to_i(2) }.reduce(&:*)
end

# 6124992
def d21032() # so ugly
  inlst = input(2103).split()
  nbits = inlst.first.length
  (0..1).map { |rank|
    (0..nbits).reduce(inlst) { |curlst, bitnum|
      nboccs = {"0" => 0, "1" => 0}.update( curlst.map{ _1[bitnum] }.tally )
      curbit = nboccs.sort_by(&:reverse).at(rank).first
      matches = curlst.select { _1[bitnum] == curbit }
      break matches.first if (matches.count == 1)
      matches
  } }.map { _1.to_i(2) }.reduce(&:*)
end


# ###########################################################################
#
# 2021 DAY 2
#
# ###########################################################################

# 1714950
def d21021()
  input(2102)
    .split
    .each_slice(2)
    .reduce([0, 0]) {|(hpos, dpth), (dir, n)|
      case dir
      when "forward" then [hpos + n.to_i, dpth]
      when "up"      then [hpos, dpth - n.to_i]
      when "down"    then [hpos, dpth + n.to_i]
      else fail end
    }
    .reduce(&:*)
end

# 1281977850
def d21022()
  input(2102)
    .split
    .each_slice(2)
    .reduce([0, 0, 0]) {|(hpos, dpth, aim), (dir, n)|
      case dir
      when "forward" then [hpos + n.to_i, dpth + n.to_i * aim, aim]
      when "up"      then [hpos, dpth, aim - n.to_i]
      when "down"    then [hpos, dpth, aim + n.to_i]
      else fail end
    }
    .tap { |(hpos, dpth, _)| break hpos * dpth }
end


# ###########################################################################
#
# 2021 DAY 1
#
# ###########################################################################

# 1676
def d21011()
  input(2101)
    .split.map(&:to_i)
    .each_cons(2).count { _1 < _2 }
end

# 1706
def d21012()
  input(2101)
    .split.map(&:to_i)
    .each_cons(3).map(&:sum)
    .each_cons(2).count { _1 < _2 }
end


# ##############################################################################

def input(id)
  IO.read("#{File.dirname(__FILE__)}/inputs/#{id}.in")
end

def debug(*args)
  puts "debug: " + args.map(&:to_s).join(" ")
  args.length == 1 ? args.first : args
end


# ##############################################################################

fail if RUBY_VERSION.to_f < 2.7

if $PROGRAM_NAME == __FILE__

  proc, *args = ARGV

  proc ||= private_methods.filter {_1.to_s.match?(/d\d{5}/) }.sort.last

  debug("call #{proc} ( #{args} )")

  fstr = Time.now
  p self.send(proc.to_sym, *args)
  fend = Time.now

  debug("took #{fend - fstr}s")

end
