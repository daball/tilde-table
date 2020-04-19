PROJECT TITLE: TildeTable
PURPOSE OF PROJECT: Complete assignment for ITEC 441, a Database II course.
VERSION or DATE: 10/27/2011
HOW TO START THIS PROJECT: Run Controller.main(null) or issue the `tildetable` command at the terminal.
HOW TO START THE SAMPLE: Run Controller.main(new Array(){"-sample"}) or issue the `tildetable --sample` command at the terminal.
AUTHOR: David Ball

Used BlueJay to develop prototype.

To execute project within BlueJ
-------------------------------

In order to run within the BlueJ interface, click the View menu and enable the Code Pad view by clicking Show Code Pad. Then inside the Code Pad pane, type:

  Controller.main(null);

This should pop out the BlueJ Terminal Window.

To execute project binaries
---------------------------

In order to run the project otherwise, use `java MainController` to launch the binary at the command-line.

Scripts have been made to simplify this.

In Windows NT 4 up to Windows 7, use:
  tildetable.cmd
In DOS/Windows 9x, use:
  tildetable.bat
In Linux/Mac, use:
  chmod +x tildetable{,.sh}
Then either:
  sh tildetable.sh
  ./tildetable.sh
  ./tildetable

To execute the sample
---------------------
Perhaps the easiest way to do this is interactively. At the main prompt, enter:
  sample

In order to run the sample within the BlueJ interface, click the View menu and enable the Code Pad view by clicking Show Code Pad. Then inside the Code Pad pane, type:

Controller.main(new String[] { "--sample" });

In order to run the sample otherwise, use `java MainController --sample` to launch the sample at the command-line.
Scripts have been made to simplify this.

In Windows NT 4 up to Windows 7, use:
  tildetable.cmd /sample
In DOS/Windows 9x, use:
  tildetable.bat /sample
In Linux/Mac, use either:
  ./tildetable.sh --sample
  ./tildetable --sample

To use application
------------------

The program was developed as a command-line interface (CLI) application. When the program launches
it displays the short user guide. Follow the directions given throughout the program workflow, it is
self-documenting.

Sample data
-----------

There are four sets of sample data. This helps ensure that the application is generalizable.

Use any these commands to load a sample data set:

  open sample-cars.txt (provided in the assignment)
  open sample-zips.txt (large zip code dataset)
  open sample-epa2010.txt (epa 2010 small truck/car test results)
  open sample-npas.txt (area codes)

It is important to note that the entire file name must be entered when loading a file, but after
the filter is loaded it will be referred to by its relation name. The relation name defaults to the
file name without the extension.

Filtering data
--------------

There are various filters in the program, each fairly well defined. To filter any of the sample data
loaded previously, use

  filter sample-cars
  filter sample-zips
  filter sample-epa2010
  filter sample-npas

The application will ask if you want to filter the relation. It's somewhat obvious how to use the program
and I've made every effort to ensure it doesn't crash while using it.

What may not be obvious is that after filtering, it will prompt for a new relation name. You may use
the sample name (overwrites the existing relation) or provide a new one. Entering a blank uses the
existing name.

Listing known relations
-----------------------

At any time, you may list named relations by using:

  show

Describing the relation
-----------------------

A relation description may be printed using:

  describe (relation-name)
  
You may use short-hand:

  desc (rel-name)

Projecting/selecting/querying the data
--------------------------------------

In order to print out the current version of the data, use:

  project (relation-name)
  project sample-cars
  project sample-zips
  project sample-epa2010
  project sample-npas
  
There is also a command alias for select->project:
  select (relation-name)
  
You may also use short hand:
  sel (rel-name)
  proj (rel-name)
  
Why did I over-complicate the assignment?
-----------------------------------------
(1) I am interested in the complexities of advanced programming and data manipulation.
(2) If applied to CSVs instead of TildeTables, the project may be useful to the worldwide community.
    I may choose to release this application under a BSD-style or GPL-style license, but I was thinking
    a nice GUI would do it justice. Also, it may be just as useful as a web application; not a bad
    way to bring in advertising revenue. (Though I would probably convert the logic to PHP.)
(3) It would be interesting to note this could be used to scrub real data, whether coming from a file
    or database. Probably not as useful in real-time, but certainly a nice way to navigate through the
    data. As far as I know, there is not another application on the planet that does what this one
    does without advanced knowledge of programming or SQL. An end-user typically is not provided the
    options in such a literal manner and offered to structure data output in such ways. This may really
    serve as a useful program to others.
(4) To see if I could. In a way, it was a challenge. In another, it almost wasn't a challenge. In fact,
    there was no point in which I said "I can't do [this] or [that]." The whole time I kept looking to
    see if there was something I couldn't do, but I have yet to find it. I think the true challenge might
    have been joining the relations. I am still a little curious about how I might implement a strategy
    for that. I really think it can be done with another filter, but that filter's job may be tricky.
    Especially since joining can be done in several ways.
    
How do I actually solve the problem presented in the assignment?
----------------------------------------------------------------
Run the sample. (See above.) If you run it during the normal command session, you can examine it
afterwards.

The sample logic is at the top of MainView.java for your consideration. There is no .copy() issued, but
.copy() is ran inside of Relation.project(...). Although it makes a copy of the object, all data is
pipelined. The only filters that materialize the view (for obvious reasons) are:
  > distinct
  > sort
  > reverse

For now these materializations occur in memory inside of certain structures.
  > distinct and sort use R-B binary trees
  > reverse walks backwards through a linked list
  
I realize that could break if a 20 GB data file were to be sorted. But please, who really has a 20 GB
tilde table? The best I could do was come up an 8 MB one, which literally sorts in less than 1 second
on my notebook. All these materializations occur inside the filter and they clean up after themselves.

All data is treated as a data stream in terms of Relation.next(). Relation.next() is blocked during
filter materialization and once sorted, reversed, etc., begins returning the first result. Once the
Relation gets fully walked, the filters are scrubbed of data remnants. In the case of reverse, clean up
is performed during the walk.

------------------------------------------------------------------------------

ITEC 441 Tilde Table Homework (Revised)

Here is a slight revision to the homework assignement due approximately October 24th.  We will
discuss the reasons for the changes Monday.

Consider the following "tilde table" that may be stored in the file cars.txt for persistence

MAKE~MODEL~TYPE~PRICE
Toyota~Camry~Sedan~18000
Toyota~Tacoma~Truck~19000
Ford~Mustang~Sport~21000
Chevrolet~Corvette~Sport~48000
Ford~F150~Truck~25000
Toyota~Highlander~SUV~35000

For our purposes tilde tables have the following attributes and limitations:
-at most 10 columns
-first row represents column names
-all columns are strings
-column values and column names are limited to 15 characters

write a java class named Relation
the class only needs two data member which will be:
  1-  A string storing the name of the text file where the relations data is stored
  2-  An arrary of strings holding the column names of the relation
That file name should be entered using the class constructor.
For now write one other method named PROJECT which accepts an array of strings representing a
list of desired column names. The method should display the projected columns.

So the main body of a driver program might look like this:

// The relation Car is associated with the cars.txt file
Relation Cars = new Relation("cars.txt");   
// A new empty relation is created with a blank assocated text file                   
Relation Answer = new Relation();
// The Relation Answer becomes a copy of the relation Cars
// Therefore the answer.txt textfile should be updated         
Answer.CopyRelation(Cars);  
// An array of column names is created
String[] DesiredColumns = {"Make","Model","Price"};
// The Relation undergoes a projection
// The answer.txt text file should mutate accordingly
Answer.Project(DesiredColumns);
// The current data is displayed on the console.
Answer.Print;

output would be:

MAKE            MODEL           PRICE
--------------- --------------- ---------------
Toyota          Camry           18000
Toyota          Tacoma          19000
Ford            Mustang         21000
Chevrolet       Corvette        48000
Ford            F150            25000
Toyota          Highlander      35000


Make your program generalizable.  That is, it should work with any tilde table.
I think we will take about three weeks for this.If you have not programmed alot take it a step at a time.

For example:

1. create your cars.txt file using notepad or another editor.
2. write a java program which opens the text file, reads a line at a time, and prints each line.
3. modify your program to use the split method or some other java method to break each row into an array of strings
   have your program loop through each row printing the individual values
4. now start modifying your program to meet the assignment requirements
------------------------------------------------------------------------
