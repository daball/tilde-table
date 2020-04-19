import java.io.*;

/**
 * Data driver suitable for Tilde Table.
 * 
 * Tilde Table is almost exactly like a Comma-Seperated Value table.
 * 
 * This first line is columns separated by tildes (~).
 * Every consecutive line are rows with cells separated by tildes (~).
 * 
 * @author David Ball
 */
public class TildeDriver extends Driver
{
    private String path;
    private BufferedReader inputFile;

    /**
     * Constructor for objects of class TildeDriver
     */
    public TildeDriver()
    {
    }

    /**
     * Opens a file with the path specified.
     * 
     * @param  path    replaces connection string, indicates file to load.
     * @return        true if opened, false otherwise, may throw an Exception if it wants to
     */
    public boolean open(String path)
    {
        this.path = path;
        try {
            this.inputFile = new BufferedReader(new FileReader(path));
            return this.isOpen();
        } catch (IOException e) {
            System.err.println("TildeDriver handled IOException: " + e.getMessage());
            return false;
        }
    }
    
    /**
     * Closes an open data connection.
     */
    public void close()
    {
        this.path = null;
        if (this.inputFile != null)
            try {
                this.inputFile.close();
            } catch (IOException e) {
                System.err.println("TildeDriver handled IOException: " + e.getMessage());
            }
        this.inputFile = null;
    }

    /**
     * Checks to see if a file is ready for read.
     * 
     * @return        true if stream is ready, false otherwise
     */
    public boolean isOpen()
    {
        if (this.path == null || this.inputFile == null)
            return false;
        try {
            return this.inputFile.ready();
        } catch (IOException e) {
            System.err.println("TildeDriver handled IOException: " + e.getMessage());
            return false;
        }
    }
    
    /**
     * Reads the next line and splits by tilde or returns null when file is not ready or no data
     * is found.
     * 
     * @return  String array with cells/columns or null.
     */
    private String[] processNextLine()
    {
        if (!this.isOpen())
            return null;
        String line = null;
        try {
            line = this.inputFile.readLine();
        } catch (IOException e) {
            System.err.println("TildeDriver handled IOException: " + e.getMessage());
        }
        if (line == null) return null;
        return line.split("~");
    }
    
    /**
     * Overloaded: Gets the default Relation.
     * 
     * @param  queryString  entirely ignored for this particular driver
     * @return  Relation or null if connection is not open or no data is found.
     */
    public Relation query(String queryString)
    {
        return query();
    }
    
    /**
     * Overloaded: Gets the default Relation.
     * 
     * @return  Relation or null if connection is not open or no data is found.
     */
    public Relation query()
    {
        String[] cols = processNextLine();
        if (cols != null && cols.length > 0)
            return new Relation(this, cols);
        else
            return null;
    }
    
    /**
     * Gets the next row.
     * 
     * @return  Row or null when no more rows.
     */
    public Row nextRow()
    {
        String[] cells = processNextLine();
        if (cells != null && cells.length > 0)
            return new Row(cells);
        else
            return null;
    }
    
    /**
     * Rewinds back to the beginning of the tilde table. (Actually closes and reopens it.)
     */
    public void rewind()
    {
        String path = this.path;
        this.close();
        this.open(path);
        //roll ahead one line for the header
        processNextLine();
    }
}