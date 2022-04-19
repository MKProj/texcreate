
use tex_rs::{elements, Latex, UserDefined};
use tex_rs::Level::*;
use crate::set;
use tex_rs::Element;

pub fn dictionary(fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> Latex {
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);
    for p in packages{
        latex.add_package(p.to_owned())
    }

    let input = UserDefined::new(r"\input{src/structure.tex}", Meta);

    let main = UserDefined::new(r#"\section*{A}

\begin{multicols}{2}

\entry{Aardvark}{ahrd-vahrk}{Noun}{A nocturnal badger-sized burrowing mammal of Africa, with long ears, a tubular snout, and a long extensible tongue, feeding on ants and termites. Also called antbear.}

\entry{Abbreviated}{uh-bree-vee-ey-tid}{Adjective}{Constituting a shorter or smaller version of}

\entry{Aberration}{ab-uh-rey-shuh n}{Noun}{The act of deviating from the ordinary, usual, or normal type.}

\entry{Above}{uh-buhv}{Preposition}{In extended space over and not touching.}

\entry{Academia}{ak-uh-dee-mee-uh}{Noun}{The environment or community concerned with the pursuit of research, education, and scholarship.}

\entry{Accomplished}{uh-kom-plisht}{Adjective}{Completed; done; effected. Highly trained or skilled in a particular activity.}

\entry{Acidophilic}{uh-sid-uh-fil-ik, as-i-duh-}{Adjective}{Biology: having an affinity for acid stains; eosinophilic. Ecology: thriving in or requiring an acid environment.}

\entry{Adaptation}{ad-uh p-tey-shuh n}{Noun}{The action or process of adapting or being adapted. Biology: The process of change by which an organism or species becomes better suited to its environment}

\entry{Adenine}{ad-n-in, -een, -ahyn}{Noun}{A compound which is one of the four constituent bases of nucleic acids. A purine derivative, it is paired with thymine in double-stranded DNA.}

\entry{Adorable}{uh-dawr-uh-buh l}{Adjective}{Inspiring great affection or delight.}

\entry{Advanced}{ad-vanst}{Adjective}{Far on or ahead in development or progress.}

\entry{Aerial}{air-ee-uh l}{Noun}{A rod, wire, or other structure by which signals are transmitted or received as part of a radio or television transmission or receiving system.}

\entry{Affordable}{uh-fawr-duh-buh l}{Adjective}{Believed to be within one's financial means.}

\entry{Agnostic}{ag-nos-tik}{Noun}{A person who holds that the existence of the ultimate cause, as God, and the essential nature of things are unknown and unknowable, or that human knowledge is limited to experience.}

\entry{Aioli}{ahy-oh-lee}{Noun}{Mayonnaise seasoned with garlic.}

\entry{Alchemy}{al-kuh-mee}{Noun}{The medieval forerunner of chemistry, concerned with the transmutation of matter, in particular with attempts to convert base metals into gold or find a universal elixir.}

\entry{Algebra}{al-juh-bruh}{Noun}{The part of mathematics in which letters and other general symbols are used to represent numbers and quantities in formulae and equations.}

\entry{Amatol}{am-uh-tawl}{Noun}{A high explosive consisting of a mixture of TNT and ammonium nitrate.}

\entry{Almanac}{awl-muh-nak}{Noun}{An annual publication containing a calendar for the coming year, the times of such events and phenomena}

\entry{Animal}{an-uh-muh l}{Noun}{A living organism which feeds on organic matter, typically having specialized sense organs and nervous system and able to respond rapidly to stimuli.}

\entry{Ascension}{auh-sen-shuh n}{Noun}{The action of rising to an important position or a higher level.}

\entry{Aspire}{uh-spahyuh r}{Verb}{Direct one's hopes or ambitions towards achieving something.}

\entry{Athlete}{ath-leet}{Noun}{a person trained or gifted in exercises or contests involving physical agility, stamina, or strength; a participant in a sport, exercise, or game requiring physical skill.}

\entry{Azobenzene}{az-oh-ben-zeen}{Noun}{A synthetic crystalline organic compound used chiefly in dye manufacture.}

\end{multicols}

%----------------------------------------------------------------------------------------
%	SECTION B
%----------------------------------------------------------------------------------------

\section*{B}

\begin{multicols}{2}

\entry{Babble}{bab-uh l}{Verb}{Talk rapidly and continuously in a foolish, excited, or incomprehensible way.}

\entry{Balance}{bal-uh ns}{Noun}{An even distribution of weight enabling someone or something to remain upright and steady. An instrument for determining weight, typically by the equilibrium of a bar with a fulcrum at the center, from each end of which is suspended a scale or pan, one holding an object of known weight, and the other holding the object to be weighed.}

\entry{Barbet}{bahr-bit}{Noun}{A large-headed, brightly coloured fruit-eating bird that has a stout bill with tufts of bristles at the base. Barbets are found on all continents, especially in the tropics.}

\entry{Beetroot}{beet-root}{Noun}{The edible dark-red spherical root of a kind of beet, eaten as a vegetable.}

\entry{Besides}{bih-sahydz}{Preposition}{In addition to; apart from.}

\entry{Bevel}{bev-uh l}{Noun}{A slope from the horizontal or vertical in carpentry and stonework; a sloping surface or edge.}

\entry{Bevel}{bev-uh l}{Noun}{A slope from the horizontal or vertical in carpentry and stonework; a sloping surface or edge.}

\entry{Biennial}{bahy-en-ee-uh l}{Adjective}{Taking place every other year.}

\entry{Bioinformatics}{bahy-oh-in-fer-mat-iks}{Noun}{The retrieval and analysis of biochemical and biological data using mathematics and computer science, as in the study of genomes. The science of collecting and analysing complex biological data such as genetic codes.}

\entry{Bleep}{bleep}{Noun}{A short high-pitched sound made by an electronic device as a signal or to attract attention.}

\entry{Blind}{blahynd}{Adjective}{Unable to see; lacking the sense of sight; sightless.}

\entry{Bonanza}{buh-nan-zuh}{Noun}{A situation which creates a sudden increase in wealth, good fortune, or profits.}

\entry{Book}{boo k}{Noun}{A written or printed work consisting of pages glued or sewn together along one side and bound in covers.}

\entry{Bran}{bran}{Noun}{The partly ground husk of wheat or other grain, separated from flour meal by sifting.}

\entry{Break}{breyk}{Verb}{Separate into pieces as a result of a blow, shock, or strain.}

\entry{Bridge}{brij}{Noun}{A structure carrying a road, path, railway, etc. across a river, road, or other obstacle. Music: The part of a stringed instrument over which the strings are stretched.}

\entry{Brioche}{bree-ohsh}{Noun}{A light sweet yeast bread typically in the form of a small round roll.}

\entry{Buzzard}{buhz-erd}{Noun}{A large hawklike bird of prey with broad wings and a rounded tail, often seen soaring in wide circles.}

\entry{Bystander}{bahy-stan-der}{Noun}{A person who is present at an event or incident but does not take part.}

\end{multicols}

%------------------------------------------------
    "#, Body);

    let structure = UserDefined::new(r#"\usepackage[top=3.5cm,bottom=3.5cm,left=3.7cm,right=4.7cm,columnsep=30pt]{geometry} % Document margins and spacings

\usepackage[utf8]{inputenc} % Required for inputting international characters
\usepackage[T1]{fontenc} % Output font encoding for international characters

\usepackage{palatino} % Use the Palatino font

\usepackage{microtype} % Improves spacing

\usepackage{multicol} % Required for splitting text into multiple columns

\usepackage[bf,sf,center]{titlesec} % Required for modifying section titles - bold, sans-serif, centered

\usepackage{fancyhdr} % Required for modifying headers and footers
\fancyhead[L]{\textsf{\rightmark}} % Top left header
\fancyhead[R]{\textsf{\leftmark}} % Top right header
\renewcommand{\headrulewidth}{1.4pt} % Rule under the header
\fancyfoot[C]{\textbf{\textsf{\thepage}}} % Bottom center footer
\renewcommand{\footrulewidth}{1.4pt} % Rule under the footer
\pagestyle{fancy} % Use the custom headers and footers throughout the document

\newcommand{\entry}[4]{\textbf{#1}\markboth{#1}{#1}\ {(#2)}\ \textit{#3}\ $\bullet$\ {#4}} % Defines the command to print each word on the page, \markboth{}{} prints the first word on the page in the top left header and the last word in the top right
    "#, Package);

    latex.set_elements(elements![input, main, structure]);
    latex.no_maketitle();
    latex
}