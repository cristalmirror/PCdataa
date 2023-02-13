require 'vidibus-sysinfo'
require 'sys-cpu'
require 'usagewatch'
include Sys

#class of the monitor system
class System_data

    #privated methods
    private
    
    #network interface
    def network_info()
        usw = Usagewatch
        loop do
            
            puts "[NETWORK down]: #{usw.uw_bandrx}"
            puts "[NETWORK uplo]: #{usw.uw_bandtx}"

        end
    end
    
    #memory information
    def mem_info()
        usw = Usagewatch        
        puts "[MEMORY used]: #{usw.uw_memused}"
        for i in usw.uw_memtop
        puts "[MEMORY top]:  #{i}"

        end
    end
    
    #technical info de la cpu
    def cpu_info_generic()
        #class that contain info of the cpu arquiteture
        puts"#{Vidibus::Sysinfo::Cpu.command}"
        
    end

    #this method print the information of the resources consumed
    def cpu_resources()

        loop do
            
            usw = Usagewatch
            #usage of the cpu
            puts "[CPU usage]: #{usw.uw_cpuused}%"
            
            
        end
        
    end

    #public methods
    public
    
    #this method select the methods according to args
    def set_options(args)

        t1 = Thread.new(args) do
            case args[0]
            when "-ci" then
                self.cpu_info_generic()
            when "-cr" then
                self.cpu_resources()
            when "-mem" then
                self.mem_info()
            when "-nw" then
                self.network_info()
            else
                puts "hubo un error"
            end
        end
        t2 = Thread.new(args) do
            case args[1]
            when "-ci" then
                self.cpu_info_generic()
            when "-cr" then
                self.cpu_resources()
            when "-mem" then
                self.mem_info()
            when "-nw" then
                self.network_info()
            else
                puts "hubo un error"
            end
        end
        t1.join
        t2.join
    end
end

#args to programa
inp=ARGV

#constructor object
pc = System_data.new()

pc.set_options(inp)


