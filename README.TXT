PROJECT TITLE: TildeTable
PURPOSE OF PROJECT: Complete assignment for ITEC 441, a Database course.
VERSION or DATE: 10/20/2011
HOW TO START THIS PROJECT: Run MainController::main().
AUTHOR: David Ball

Used BlueJay to develop prototype.

------------------------------------------------------------------------
ITEC 441 Tilde Table Homework (Revised)

Here is a slight revision to the homework assignement due approximately October 24th.  We will discuss the reasons for the changes Monday.

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
For now write one other method named PROJECT which accepts an array of strings representing a list of desired column names.
The method should display the projected columns.
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