import java.util.TreeMap;

/**
 * Orchestrates the program activities.
 * 
 * @author David Ball
 */
public class Controller
{
    /**
     * Holds all the Relation objects for the application.
     */
    public static TreeMap<String,Relation> relations = new TreeMap<String,Relation>();

    /**
     * Main entry-point. Does the work of the assignment.
     * 
     * @param  args   any command-line args
     * @return     0, no errors
     */
    public static void main(String[] args)
    {
        if (args != null && args.length > 0)
        {
            for (String arg: args)
            {
                if (arg.trim().toLowerCase().endsWith("sample"))
                {
                    MainView.runSample();
                    return;
                }
            }
        }
        
        MainView.runLoop(args);
        
        //after exit, be sure to clean up by closing any files left open
        for (Relation relation: relations.values())
            relation.close();
        relations.clear();
    }
    
    /**
     * Returns the relation by the name.
     */
    public static Relation getRelation(String relationName)
    {
        relationName = relationName.toUpperCase();
        Relation relation = null;
        if (relations.containsKey(relationName))
        {
            relation = relations.get(relationName);
        }
        return relation;
    }
}