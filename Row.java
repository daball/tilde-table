/**
 * Row is the part of Relation for each table row.
 * 
 * @author David Ball
 */
public class Row
{
    private String[] cells;

    /**
     * Constructor for objects of class Row
     */
    Row(String[] cells)
    {
        this.cells = cells;
    }

    /**
     * Returns the value of the cell at index or null if index is out of range.
     * 
     * @param  index   the cell index to retrieve data
     * @return     the data is cell at index or null if index is out of range
     */
    public String get(int index)
    {
        // put your code here
        return this.cells[index];
    }
    
    /**
     * Returns the number of columns in the row.
     */
    public int columnCount()
    {
        return this.cells.length;
    }
}