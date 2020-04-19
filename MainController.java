/**
 * Orchestrates the program activities.
 * 
 * @author David Ball
 */
public class MainController
{
    /**
     * Main entry-point. Does the work of the assignment.
     * 
     * @param  args   any command-line args
     * @return     0, no errors
     */
    public static int main(String[] args)
    {
        return CmdLineView.runLoop(args);
    }
}