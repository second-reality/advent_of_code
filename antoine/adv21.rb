#!/usr/bin/env ruby

# ##############################################################################

# load "adv21.rb" ; d21011()

# ##############################################################################

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
  IO.read("inputs/#{id}.in")
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

  p self.send(proc.to_sym, *args)

end
