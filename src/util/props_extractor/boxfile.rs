use regex::Regex;

#[derive(Debug, Clone)]
pub struct TextPos {
    pub text: String,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

fn split_row(txt: &str) -> Vec<&str> {
    let newline_re = Regex::new(r"\u21b5|\n").unwrap();

    newline_re.split(txt).filter(|x| *x != "").collect()
}

fn split_el(txt: &str) -> Vec<&str> {
    let trim_re = Regex::new(r"[\s]+").unwrap();

    trim_re.split(txt).filter(|x| *x != "").collect()
}

fn each_text_pos(vec: Vec<&str>) -> TextPos {
    TextPos {
        text: vec.get(0).unwrap().to_string(),
        left: vec.get(1).unwrap().parse().unwrap(),
        bottom: vec.get(2).unwrap().parse().unwrap(),
        right: vec.get(3).unwrap().parse().unwrap(),
        top: vec.get(4).unwrap().parse().unwrap(),
    }
}

pub fn run(txt: &str) -> Vec<TextPos> {
    let res = split_row(txt);

    let res = res
        .iter()
        .map(|txt| split_el(*txt).clone())
        .collect::<Vec<Vec<&str>>>();

    let res = res
        .iter()
        .map(|x| each_text_pos(x.to_vec()))
        .collect::<Vec<TextPos>>();

    res
}

mod tests {
    use super::*;

    const CASE1: &str = "~ 790 1004 2145 1011 0
    ~ 388 994 754 1001 0
    N 293 960 321 988 0
    人 703 969 722 999 0
    | 753 984 783 1007 0
    和 492 898 561 933 0
    a 1568 913 1855 978 0
    s 1722 913 1787 978 0
    转 1767 854 1795 883 0
    生 1797 855 1825 883 0
    前 1832 854 1854 883 0
    转 1996 854 2024 883 0
    生 2026 854 2084 883 0
    后 2061 844 2087 887 0
    L 816 812 834 829 0
    A 858 823 867 832 0
    虑 813 786 841 814 0
    就 852 785 872 814 0
    总 1587 798 1628 823 0
    成 1641 798 1663 823 0
    长 1662 791 1689 838 0
    ( 1732 797 1739 824 0
    3 1740 802 1753 823 0
    . 1756 802 1761 806 0
    0 1763 802 1776 822 0
    9 1778 802 1791 823 0
    1 1794 802 1805 822 0
    - 1808 809 1820 814 0
    5 1822 801 1835 822 0
    . 1838 802 1842 806 0
    1 1845 802 1857 822 0
    5 1859 802 1888 822 0
    4 1890 797 1896 824 0
    ) 1894 797 1896 824 0
    。 1926 791 1945 838 0
    ( 1962 797 1968 824 0
    3 1970 802 1982 823 0
    . 1985 802 2005 822 0
    5 2000 797 2012 824 0
    5 2007 801 2020 822 0
    3 2022 801 2035 823 0
    ~ 2037 809 2049 815 0
    7 2051 802 2065 822 0
    . 2067 802 2072 806 0
    2 2074 802 2087 823 0
    1 2090 802 2101 822 0
    4 2102 802 2117 822 0
    ) 2119 797 2125 824 0
    4 1455 778 1542 807 0
    生 1589 755 1613 780 0
    命 1615 754 1657 780 0
    成 1669 755 1690 780 0
    长 1690 747 1726 795 0
    ( 1747 754 1754 781 0
    0 1755 758 1769 779 0
    . 1771 758 1775 763 0
    9 1778 758 1791 779 0
    4 1792 759 1806 779 0
    ~ 1808 766 1820 771 0
    1 1823 759 1835 779 0
    . 1838 759 1842 763 0
    5 1845 758 1858 779 0
    6 1860 758 1873 780 0
    8 1874 758 1888 779 0
    ) 1890 753 1896 781 0
    ( 1962 754 1968 781 0
    1 1971 759 1982 779 0
    . 1985 758 1990 763 0
    0 1992 758 2006 779 0
    8 2007 758 2020 779 0
    1 2022 759 2035 779 0
    - 2037 766 2049 771 0
    2 2052 759 2065 780 0
    . 2067 758 2072 763 0
    1 2075 758 2087 779 0
    9 2089 758 2102 779 0
    5 2104 758 2116 779 0
    ) 2119 754 2125 781 0
    人 1125 698 1165 759 0
    > 1169 690 1211 750 0
    攻 1588 712 1614 737 0
    击 1616 711 1656 737 0
    成 1669 712 1689 737 0
    长 1689 703 1722 751 0
    ( 1733 710 1739 738 0
    1 1741 715 1753 736 0
    . 1756 715 1761 720 0
    1 1764 715 1776 736 0
    9 1778 715 1791 736 0
    9 1792 715 1806 736 0
    ~ 1808 723 1820 728 0
    1 1823 715 1835 736 0
    . 1838 715 1842 720 0
    9 1845 715 1858 736 0
    9 1860 715 1873 736 0
    9 1874 715 1888 736 0
    ) 1890 710 1896 738 0
    ( 1962 710 1968 738 0
    1 1970 715 1982 736 0
    . 1985 715 1990 720 0
    3 1992 715 2005 736 0
    7 2007 715 2020 736 0
    8 2022 715 2035 736 0
    ~ 2037 723 2049 728 0
    2 2051 715 2065 736 0
    . 2067 715 2072 720 0
    7 2074 715 2087 736 0
    9 2089 715 2102 736 0
    8 2104 715 2117 736 0
    ) 2119 710 2125 738 0
    w 1126 649 1140 670 0
    和 1204 675 1228 700 0
    动 1453 653 1507 682 0
    渡 1509 654 1541 686 0
    肖 1540 649 1565 707 0
    防 1597 668 1614 694 0
    御 1615 668 1656 694 0
    成 1669 669 1692 694 0
    长 1691 649 1718 707 0
    ( 1747 667 1754 695 0
    0 1755 672 1769 693 0
    . 1771 672 1776 677 0
    2 1778 672 1791 693 0
    5 1793 672 1806 693 0
    - 1808 680 1820 685 0
    0 1822 672 1836 693 0
    . 1838 672 1842 677 0
    4 1844 672 1859 693 0
    1 1860 672 1872 693 0
    7 1874 672 1888 693 0
    ) 1890 667 1896 695 0
    。 1927 649 1946 707 0
    ( 1962 667 1968 695 0
    0 1970 672 1983 693 0
    . 1985 672 1990 677 0
    2 1992 672 2005 693 0
    8 2007 672 2020 693 0
    7 2022 672 2035 693 0
    ~ 2037 680 2049 685 0
    0 2051 672 2065 693 0
    . 2067 672 2072 677 0
    5 2074 672 2087 693 0
    8 2089 672 2102 693 0
    3 2104 672 2116 693 0
    ) 2119 667 2125 695 0
    三 1063 617 1087 642 0
    可 1084 577 1184 649 0
    敏 1588 625 1694 651 0
    所 1626 617 1650 665 0
    成 1650 625 1689 651 0
    长 1689 617 1716 665 0
    。 1725 617 1734 665 0
    ( 1747 624 1753 652 0
    0 1755 629 1769 650 0
    . 1771 629 1776 634 0
    7 1778 629 1791 649 0
    0 1793 629 1806 650 0
    2 1807 629 1821 650 0
    ~ 1822 637 1835 642 0
    1 1838 629 1850 649 0
    . 1853 629 1857 634 0
    1 1860 629 1872 649 0
    7 1875 624 1896 652 0
    ) 1893 624 1896 652 0
    ( 1962 624 1968 652 0
    0 1970 629 1983 650 0
    . 1985 629 1990 634 0
    8 1992 629 2005 650 0
    0 2007 629 2020 650 0
    7 2022 629 2035 649 0
    ~ 2037 637 2049 642 0
    1 2052 629 2064 649 0
    . 2067 629 2072 634 0
    6 2074 629 2087 650 0
    3 2089 629 2102 650 0
    8 2104 629 2117 650 0
    ) 2119 624 2125 652 0
    人 1171 576 1206 620 0
    元 1587 562 1618 591 0
    素 1630 562 1652 594 0
    ~ 337 558 590 848 0
    等 1123 501 1206 579 0
    本 1244 461 1310 484 0
    区 1592 502 1614 529 0
    二 1630 499 1645 527 0
    一 1656 499 1684 527 0
    - 1684 485 1704 533 0
    一 1697 499 1712 527 0
    - 1724 499 1734 527 0
    一 1742 499 1779 527 0
    一 1787 499 1801 527 0
    一 1812 499 1846 527 0
    ~ 324 379 341 537 0
    ~ 471 379 732 537 0
    局 341 235 471 354 0
    ~ 306 178 307 208 0
    > 152 0 178 0 0
    下 145 81 257 116 0
    县 181 69 204 123 0
    , 204 81 231 116 0
    放 231 81 263 116 0
    生 269 85 303 111 0
    二 312 82 357 116 0
    ~ 815 64 1958 482 0";

    #[test]
    fn row() {
        let res = split_row(CASE1);

        let res = res
            .iter()
            .map(|txt| split_el(*txt).clone())
            .collect::<Vec<Vec<&str>>>();

        let res = res
            .iter()
            .map(|x| each_text_pos(x.to_vec()))
            .collect::<Vec<TextPos>>();

        println!("{:?}", res)
    }
}
