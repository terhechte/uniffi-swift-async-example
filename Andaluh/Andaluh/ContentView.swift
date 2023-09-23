import SwiftUI

struct ContentView: View {
    @State private var output: String? = nil
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            if let output {
                Text(output)
            }
        }
        .padding()
        .onAppear {
            Task {
                let h = TicketHandler()
                let o = await h.getTickets()
                print(o)
            }
        }
    }
}

#Preview {
    ContentView()
}
