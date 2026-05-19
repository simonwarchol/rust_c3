/// All C3 color-name tokens in fixed order (length **W** for composite keys).
pub static C3_TERM_STRS: &[&str] = &[
    "green", "blue", "purple", "red", "pink", "yellow",
    "orange", "brown", "teal", "lightblue", "grey", "limegreen",
    "magenta", "lightgreen", "brightgreen", "skyblue", "cyan", "turquoise",
    "darkblue", "darkgreen", "aqua", "olive", "navyblue", "lavender",
    "fuchsia", "black", "royalblue", "violet", "hotpink", "tan",
    "forestgreen", "lightpurple", "neongreen", "yellowgreen", "maroon", "darkpurple",
    "salmon", "peach", "beige", "lime", "seafoamgreen", "mustard",
    "brightblue", "lilac", "seagreen", "palegreen", "bluegreen", "mint",
    "lightbrown", "mauve", "darkred", "greyblue", "burntorange", "darkpink",
    "indigo", "periwinkle", "bluegrey", "lightpink", "aquamarine", "gold",
    "brightpurple", "grassgreen", "redorange", "bluepurple", "greygreen", "kellygreen",
    "puke", "rose", "darkteal", "babyblue", "paleblue", "greenyellow",
    "brickred", "lightgrey", "darkgrey", "white", "brightpink", "chartreuse",
    "purpleblue", "royalpurple", "burgundy", "goldenrod", "darkbrown", "lightorange",
    "darkorange", "redbrown", "paleyellow", "plum", "offwhite", "pinkpurple",
    "darkyellow", "lightyellow", "mustardyellow", "brightred", "peagreen", "khaki",
    "orangered", "crimson", "deepblue", "springgreen", "cream", "palepink",
    "yelloworange", "deeppurple", "pinkred", "pastelgreen", "sand", "rust",
    "lightred", "taupe", "armygreen", "robinseggblue", "huntergreen", "greenblue",
    "lightteal", "cerulean", "flesh", "orangebrown", "slateblue", "slate",
    "coral", "blueviolet", "ochre", "leafgreen", "electricblue", "seablue",
    "midnightblue", "steelblue", "brick", "palepurple", "mediumblue", "burntsienna",
    "darkmagenta", "eggplant", "sage", "darkturquoise", "puce", "bloodred",
    "neonpurple", "mossgreen", "terracotta", "oceanblue", "yellowbrown", "brightyellow",
    "dustyrose", "applegreen", "neonpink", "skin", "cornflowerblue", "lightturquoise",
    "wine", "deepred", "azure",
];

/// Borrowed term strings (zero-copy); length is **W**.
#[inline]
pub fn get_c3_terms() -> &'static [&'static str] {
    C3_TERM_STRS
}
